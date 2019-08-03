use kube::api::{Object, Reflector, KubeObject, Api};
use kube::config::Configuration;
use kube::client::APIClient;
use kube::Result;
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use k8s_openapi::api::core::v1::{ServiceSpec, ServiceStatus};
use serde::de::DeserializeOwned;
use super::{ObjectRepository, Identifier};

#[derive(Clone)]
pub struct ApiObjectRepository {
    pods_reflector: Reflector<Object<PodSpec, PodStatus>>,
    service_reflector: Reflector<Object<ServiceSpec, ServiceStatus>>,
}

impl ApiObjectRepository {
    pub fn new(kube_config: Configuration) -> Result<Self> {
        let client = APIClient::new(kube_config);
        let pod_reflector = ApiObjectRepository::initialize_reflector(Api::v1Pod(client.clone()))?;
        let service_reflector = ApiObjectRepository::initialize_reflector(Api::v1Service(client))?;

        Ok(ApiObjectRepository {
            pods_reflector: pod_reflector,
            service_reflector,
        })
    }

    fn initialize_reflector<K: 'static + Send + Sync + Clone + DeserializeOwned + KubeObject>(api: Api<K>) -> Result<Reflector<K>> {
        let mut pod_reflect = Reflector::new(api);
        pod_reflect = pod_reflect.init()?;
        let pod_reflect_updater = pod_reflect.clone();

        std::thread::spawn(move || {
            loop {
                if let Err(e) = pod_reflect_updater.poll() {
                    println!("Error while updating pods: {}", e.to_string());
                }
            }
        });

        Ok(pod_reflect)
    }

    fn find_object<K: 'static + Send + Sync + Clone + DeserializeOwned + KubeObject>(id: &Identifier, reflector: &Reflector<K>) -> Option<K> {
        let objs = reflector.read().ok()?;

        objs.iter()
            .find_map(|o| {
                if id.matches_with(o.1.meta()) {
                    Some(o.1.clone())
                } else {
                    None
                }
            })
    }

    fn all_objects<K: 'static + Send + Sync + Clone + DeserializeOwned + KubeObject>(reflector: &Reflector<K>) -> Vec<K> {
        reflector.read()
            .unwrap_or_default()
            .iter()
            .map(|element| element.1.clone())
            .collect()
    }
}

impl ObjectRepository for ApiObjectRepository {
    fn pod(&self, id: &Identifier) -> Option<Object<PodSpec, PodStatus>> {
        Self::find_object(id, &self.pods_reflector)
    }

    fn pods(&self) -> Vec<Object<PodSpec, PodStatus>> {
        Self::all_objects(&self.pods_reflector)
    }

    fn service(&self, id: &Identifier) -> Option<Object<ServiceSpec, ServiceStatus>> {
        Self::find_object(id, &self.service_reflector)
    }

    fn services(&self) -> Vec<Object<ServiceSpec, ServiceStatus>> {
        Self::all_objects(&self.service_reflector)
    }
}