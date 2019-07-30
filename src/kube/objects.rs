use kube::api::{Object, Reflector};
use std::collections::HashMap;
use kube::config::Configuration;
use std::sync::Arc;
use kube::client::APIClient;
use kube::Result;

pub struct ObjectRepository {
    pods_reflector: Reflector<Object<PodSpec, PodStatus>>,
}

pub struct Identifier {
    name: String,
    namespace: String,
}

impl ObjectRepository {
    pub fn new(kube_config: Configuration) -> Result<Self> {
        let client = APIClient::new(kube_config);
        let pod_reflect = Reflector::new(client)?;

        Ok(ObjectRepository {
            pods_reflector: pod_reflect,
        })
    }

    pub fn pod(&self, id: &Identifier) -> Option<Arc<Object<PodSpec, PodStatus>>> {
        self.pods.get(&id).cloned()
    }

    pub fn pods(&self) ->
}