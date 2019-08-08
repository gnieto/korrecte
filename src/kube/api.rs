use kube::api::{Object, Reflector, KubeObject, Api, RawApi, ObjectMeta};
use kube::config::Configuration;
use kube::client::APIClient;
use kube::Result;
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use k8s_openapi::api::core::v1::{ServiceSpec, ServiceStatus};
use serde::de::DeserializeOwned;
use super::{ObjectRepository, Identifier};
use std::sync::Arc;
use std::marker::PhantomData;
use serde::{Deserialize, Deserializer};
use std::result::Result as CoreResult;

#[derive(Clone)]
pub struct ApiObjectRepository {
    pods_reflector: Reflector<ArcKubeObject<Object<PodSpec, PodStatus>>>,
    service_reflector: Reflector<Object<ServiceSpec, ServiceStatus>>,
}

impl ApiObjectRepository {
    pub fn new(kube_config: Configuration) -> Result<Self> {
        let client = APIClient::new(kube_config);
        let pod_api = create_arc_v1_pod(client.clone());
        let pod_reflector = ApiObjectRepository::initialize_reflector(pod_api)?;
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
                if id.matches_with(o.meta()) {
                    Some(o.clone())
                } else {
                    None
                }
            })
    }

    fn all_objects<K: 'static + Send + Sync + Clone + DeserializeOwned + KubeObject>(reflector: &Reflector<K>) -> Vec<K> {
        reflector.read()
            .unwrap_or_default()
            .iter()
            .map(|element| element.clone())
            .collect()
    }

}

impl ObjectRepository for ApiObjectRepository {
    fn pods(&self) -> Vec<Arc<Object<PodSpec, PodStatus>>> {
        self.pods_reflector.read()
            .unwrap_or_default()
            .iter()
            .map(|entry| entry.get())
            .collect()
    }

    fn service(&self, id: &Identifier) -> Option<Object<ServiceSpec, ServiceStatus>> {
        Self::find_object(id, &self.service_reflector)
    }

    fn services(&self) -> Vec<Object<ServiceSpec, ServiceStatus>> {
        Self::all_objects(&self.service_reflector)
    }
}

fn create_arc_v1_pod(client: APIClient) -> Api<ArcKubeObject<Object<PodSpec, PodStatus>>> {
    Api::new(RawApi::v1Pod(), client)
}

#[derive(Clone)]
struct ArcKubeObject<K: Clone> {
    inner: Arc<K>,
}

impl<K: KubeObject + Clone + DeserializeOwned> ArcKubeObject<K> {
    pub fn new(inner: K) -> Self {
        ArcKubeObject {
            inner: Arc::new(inner),
        }
    }

    pub fn get(&self) -> Arc<K> {
        self.inner.clone()
    }
}

impl<K: KubeObject + Clone + DeserializeOwned> KubeObject for ArcKubeObject<K> {
    fn meta(&self) -> &ObjectMeta {
        self.inner.meta()
    }
}

impl<'de, T: DeserializeOwned + Clone + KubeObject> Deserialize<'de> for ArcKubeObject<T> {
    fn deserialize<D>(deserializer: D) -> CoreResult<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(ArcKubeObject::new)
    }
}