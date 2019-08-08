use kube::api::{ObjectMeta, Object};
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use k8s_openapi::api::core::v1::{ServiceSpec, ServiceStatus};
use serde::Deserialize;
use std::sync::Arc;

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
    fn pod(&self, id: &Identifier) -> Option<Arc<Object<PodSpec, PodStatus>>> {
        self.pods()
            .iter()
            .filter(|p| id.matches_with(&p.metadata))
            .next()
            .cloned()
    }

    fn pods(&self) -> Vec<Arc<Object<PodSpec, PodStatus>>>;

    fn service(&self, id: &Identifier) -> Option<Object<ServiceSpec, ServiceStatus>> {
        self.services()
            .iter()
            .filter(|s| id.matches_with(&s.metadata))
            .next()
            .cloned()
    }
    fn services(&self) -> Vec<Object<ServiceSpec, ServiceStatus>>;
}

#[derive(Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum KubeObjectType {
    Pod(Object<PodSpec, PodStatus>),
    Service(Object<ServiceSpec, ServiceStatus>),
}