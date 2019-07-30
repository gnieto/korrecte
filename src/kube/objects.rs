use kube::api::{Object, Reflector, KubeObject, ObjectMeta, Api};
use kube::config::Configuration;
use kube::client::APIClient;
use kube::Result;
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct ObjectRepository {
    pods_reflector: Reflector<Object<PodSpec, PodStatus>>,
}

pub struct Identifier {
    name: String,
    namespace: Option<String>,
}

impl Identifier {
    pub fn matches_with(&self, meta: &ObjectMeta) -> bool {
        meta.name == self.name &&
            meta.namespace == self.namespace
    }
}

impl ObjectRepository {
    pub fn new(kube_config: Configuration) -> Result<Self> {
        let client = APIClient::new(kube_config);
        let resource = Api::v1Pod(client);
        let mut pod_reflect = Reflector::new(resource);
        pod_reflect = pod_reflect.init()?;
        let pod_reflect_updater = pod_reflect.clone();

        std::thread::spawn(move || {
            loop {
                if let Err(e) = pod_reflect_updater.poll() {
                    println!("Error while updating pods: {}", e.to_string());
                }
            }
        });

        Ok(ObjectRepository {
            pods_reflector: pod_reflect,
        })
    }

    pub fn pod(&self, id: &Identifier) -> Option<Object<PodSpec, PodStatus>> {
        let pods = self.pods_reflector.read().ok()?;

        pods.iter()
            .find_map(|o| {
                if id.matches_with(o.1.meta()) {
                    Some(o.1.clone())
                } else {
                    None
                }
            })
    }

    pub fn pods(&self) -> Vec<Object<PodSpec, PodStatus>> {
        self.pods_reflector.read()
            .unwrap_or(BTreeMap::new())
            .iter()
            .map(|element| element.1.clone())
            .collect::<Vec<Object<PodSpec, PodStatus>>>()


    }
}