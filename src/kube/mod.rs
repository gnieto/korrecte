use kube::api::{ObjectMeta, Object};
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use k8s_openapi::api::core::v1::{ServiceSpec, ServiceStatus};
use serde::Deserialize;

pub mod api;
pub mod file;

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

pub trait ObjectRepository {
    fn pod(&self, id: &Identifier) -> Option<Object<PodSpec, PodStatus>>;
    fn pods(&self) -> Vec<Object<PodSpec, PodStatus>>;

    fn service(&self, id: &Identifier) -> Option<Object<ServiceSpec, ServiceStatus>>;
    fn services(&self) -> Vec<Object<ServiceSpec, ServiceStatus>>;
}

#[derive(Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum KubeObjectType {
    Pod(Object<PodSpec, PodStatus>),
    Service(Object<ServiceSpec, ServiceStatus>),
}