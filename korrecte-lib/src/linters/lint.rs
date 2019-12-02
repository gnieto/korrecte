use crate::linters::evaluator::Context;
use anyhow::{anyhow, Result};

pub trait Lint {
    fn v1_node(&self, _node: &k8s_openapi::api::core::v1::Node, _context: &Context) {}
    fn v1_pod(&self, _pod: &k8s_openapi::api::core::v1::Pod, _context: &Context) {}
    fn v1_service(&self, _service: &k8s_openapi::api::core::v1::Service, _context: &Context) {}
    fn v1_daemon_set(
        &self,
        _daemon_set: &k8s_openapi::api::apps::v1::DaemonSet,
        _context: &Context,
    ) {
    }
    fn v1_deployment(
        &self,
        _deployment: &k8s_openapi::api::apps::v1::Deployment,
        _context: &Context,
    ) {
    }
    fn v1_replica_set(
        &self,
        _replica_set: &k8s_openapi::api::apps::v1::ReplicaSet,
        _context: &Context,
    ) {
    }
    fn v1_stateful_set(
        &self,
        _stateful_set: &k8s_openapi::api::apps::v1::StatefulSet,
        _context: &Context,
    ) {
    }
    fn v1beta1_pod_disruption_budget(
        &self,
        _pod_disruption_budget: &k8s_openapi::api::policy::v1beta1::PodDisruptionBudget,
        _context: &Context,
    ) {
    }
    fn v1_horizontal_pod_autoscaler(
        &self,
        _horizontal_pod_autoscaler: &k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler,
        _context: &Context,
    ) {
    }
    fn v1beta1_ingress(
        &self,
        _ingress: &k8s_openapi::api::networking::v1beta1::Ingress,
        _context: &Context,
    ) {
    }
    fn v1_cluster_role(
        &self,
        _cluster_role: &k8s_openapi::api::rbac::v1::ClusterRole,
        _context: &Context,
    ) {
    }
    fn v1_role(&self, _role: &k8s_openapi::api::rbac::v1::Role, _context: &Context) {}

    fn object(&self, object: &KubeObjectType, context: &Context) {
        match object {
            KubeObjectType::V1Node(ref o) => self.v1_node(o, context),
            KubeObjectType::V1Pod(ref o) => self.v1_pod(o, context),
            KubeObjectType::V1Service(ref o) => self.v1_service(o, context),
            KubeObjectType::V1DaemonSet(ref o) => self.v1_daemon_set(o, context),
            KubeObjectType::V1Deployment(ref o) => self.v1_deployment(o, context),
            KubeObjectType::V1ReplicaSet(ref o) => self.v1_replica_set(o, context),
            KubeObjectType::V1StatefulSet(ref o) => self.v1_stateful_set(o, context),
            KubeObjectType::V1beta1PodDisruptionBudget(ref o) => {
                self.v1beta1_pod_disruption_budget(o, context)
            }
            KubeObjectType::V1HorizontalPodAutoscaler(ref o) => {
                self.v1_horizontal_pod_autoscaler(o, context)
            }
            KubeObjectType::V1beta1Ingress(ref o) => self.v1beta1_ingress(o, context),
            KubeObjectType::V1ClusterRole(ref o) => self.v1_cluster_role(o, context),
            KubeObjectType::V1Role(ref o) => self.v1_role(o, context),
        }
    }
}

#[allow(unused)]
pub enum KubeObjectType {
    V1Node(Box<k8s_openapi::api::core::v1::Node>),
    V1Pod(Box<k8s_openapi::api::core::v1::Pod>),
    V1Service(Box<k8s_openapi::api::core::v1::Service>),
    V1DaemonSet(Box<k8s_openapi::api::apps::v1::DaemonSet>),
    V1Deployment(Box<k8s_openapi::api::apps::v1::Deployment>),
    V1ReplicaSet(Box<k8s_openapi::api::apps::v1::ReplicaSet>),
    V1StatefulSet(Box<k8s_openapi::api::apps::v1::StatefulSet>),
    V1beta1PodDisruptionBudget(Box<k8s_openapi::api::policy::v1beta1::PodDisruptionBudget>),
    V1HorizontalPodAutoscaler(Box<k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler>),
    V1beta1Ingress(Box<k8s_openapi::api::networking::v1beta1::Ingress>),
    V1ClusterRole(Box<k8s_openapi::api::rbac::v1::ClusterRole>),
    V1Role(Box<k8s_openapi::api::rbac::v1::Role>),
}

impl KubeObjectType {
    pub fn from_yaml(
        yaml: &str,
        api_version: &str,
        kind: &str,
    ) -> Result<KubeObjectType, anyhow::Error> {
        let (ty, version) = if api_version.contains('/') {
            let mut parts = api_version.split('/');
            (parts.next().unwrap(), parts.next().unwrap())
        } else {
            ("core", api_version)
        };

        match (ty, version, kind) {
            ("core", "v1", "Node") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::V1Node(object))
            }

            ("core", "v1", "Pod") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::V1Pod(object))
            }

            ("core", "v1", "Service") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::V1Service(object))
            }

            ("apps", "v1", "DaemonSet") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::V1DaemonSet(object))
            }

            ("apps", "v1", "Deployment") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::V1Deployment(object))
            }

            ("apps", "v1", "ReplicaSet") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::V1ReplicaSet(object))
            }

            ("apps", "v1", "StatefulSet") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::V1StatefulSet(object))
            }

            ("policy", "v1beta1", "PodDisruptionBudget") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::V1beta1PodDisruptionBudget(object))
            }

            ("autoscaling", "v1", "HorizontalPodAutoscaler") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::V1HorizontalPodAutoscaler(object))
            }

            ("networking.k8s.io", "v1beta1", "Ingress") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::V1beta1Ingress(object))
            }

            ("rbac.authorization.k8s.io", "v1", "ClusterRole") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::V1ClusterRole(object))
            }

            ("rbac.authorization.k8s.io", "v1", "Role") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::V1Role(object))
            }
            _ => Err(anyhow!("Could not decode the given object type")),
        }
    }
}
