use kube::api::{ObjectMeta, Object};
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use k8s_openapi::api::core::v1::{ServiceSpec, ServiceStatus};
use k8s_openapi::api::apps::v1::{DeploymentSpec, DeploymentStatus};
use k8s_openapi::api::autoscaling::v1::{HorizontalPodAutoscalerSpec, HorizontalPodAutoscalerStatus};
use k8s_openapi::api::policy::v1beta1::{PodDisruptionBudgetSpec, PodDisruptionBudgetStatus};
use crate::linters::KubeObjectType;

pub mod api;
// pub mod file;

pub struct Identifier {
    name: String,
    namespace: Option<String>,
}

impl From<String> for Identifier {
    fn from(s: String) -> Self {
        Identifier {
            name: s,
            namespace: None,
        }
    }
}

impl From<ObjectMeta> for Identifier {
    fn from(meta: ObjectMeta) -> Self {
        Identifier {
            name: meta.name,
            namespace: meta.namespace,
        }
    }
}

impl Identifier {
    pub fn matches_with(&self, meta: &ObjectMeta) -> bool {
        meta.name == self.name &&
            meta.namespace == self.namespace
    }
}

pub trait ObjectRepository {
    fn pod(&self, id: &Identifier) -> Option<Object<PodSpec, PodStatus>> {
        self.pods()
            .iter()
            .filter(|p| id.matches_with(&p.metadata))
            .next()
            .cloned()
    }

    fn service(&self, id: &Identifier) -> Option<Object<ServiceSpec, ServiceStatus>> {
        self.services()
            .iter()
            .filter(|s| id.matches_with(&s.metadata))
            .next()
            .cloned()
    }

    fn deployment(&self, id: &Identifier) -> Option<Object<DeploymentSpec, DeploymentStatus>> {
        self.deployments()
            .iter()
            .filter(|s| id.matches_with(&s.metadata))
            .next()
            .cloned()
    }

    fn pods(&self) -> Vec<Object<PodSpec, PodStatus>>;
    fn services(&self) -> Vec<Object<ServiceSpec, ServiceStatus>>;
    fn pod_disruption_budgets(&self) -> Vec<Object<PodDisruptionBudgetSpec, PodDisruptionBudgetStatus>>;
    fn deployments(&self) -> Vec<Object<DeploymentSpec, DeploymentStatus>>;
    fn horizontal_pod_autoscaler(&self) -> Vec<Object<HorizontalPodAutoscalerSpec, HorizontalPodAutoscalerStatus>>;
}

pub trait NewObjectRepository {
    fn all(&self) -> &Vec<KubeObjectType>;
    fn find(&self, id: &Identifier) -> Option<&KubeObjectType>;
}