use crate::linters::evaluator::Context;
use anyhow::{anyhow, Result};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

pub trait Lint {
    fn name(&self) -> &str;
    fn core_v1_node(&self, _node: &k8s_openapi::api::core::v1::Node, _context: &Context) {}
    fn core_v1_pod(&self, _pod: &k8s_openapi::api::core::v1::Pod, _context: &Context) {}
    fn core_v1_service(&self, _service: &k8s_openapi::api::core::v1::Service, _context: &Context) {}
    fn apps_v1_daemon_set(
        &self,
        _daemon_set: &k8s_openapi::api::apps::v1::DaemonSet,
        _context: &Context,
    ) {
    }
    fn apps_v1_deployment(
        &self,
        _deployment: &k8s_openapi::api::apps::v1::Deployment,
        _context: &Context,
    ) {
    }
    fn apps_v1_replica_set(
        &self,
        _replica_set: &k8s_openapi::api::apps::v1::ReplicaSet,
        _context: &Context,
    ) {
    }
    fn apps_v1_stateful_set(
        &self,
        _stateful_set: &k8s_openapi::api::apps::v1::StatefulSet,
        _context: &Context,
    ) {
    }
    fn policy_v1beta1_pod_disruption_budget(
        &self,
        _pod_disruption_budget: &k8s_openapi::api::policy::v1beta1::PodDisruptionBudget,
        _context: &Context,
    ) {
    }
    fn autoscaling_v1_horizontal_pod_autoscaler(
        &self,
        _horizontal_pod_autoscaler: &k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler,
        _context: &Context,
    ) {
    }
    fn autoscaling_v2beta1_horizontal_pod_autoscaler(
        &self,
        _horizontal_pod_autoscaler: &k8s_openapi::api::autoscaling::v2beta1::HorizontalPodAutoscaler,
        _context: &Context,
    ) {
    }
    fn autoscaling_v2beta2_horizontal_pod_autoscaler(
        &self,
        _horizontal_pod_autoscaler: &k8s_openapi::api::autoscaling::v2beta2::HorizontalPodAutoscaler,
        _context: &Context,
    ) {
    }
    fn networking_v1beta1_ingress(
        &self,
        _ingress: &k8s_openapi::api::networking::v1beta1::Ingress,
        _context: &Context,
    ) {
    }
    fn extensions_v1beta1_ingress(
        &self,
        _ingress: &k8s_openapi::api::extensions::v1beta1::Ingress,
        _context: &Context,
    ) {
    }
    fn rbac_v1_cluster_role(
        &self,
        _cluster_role: &k8s_openapi::api::rbac::v1::ClusterRole,
        _context: &Context,
    ) {
    }
    fn rbac_v1_role(&self, _role: &k8s_openapi::api::rbac::v1::Role, _context: &Context) {}

    fn object(&self, object: &KubeObjectType, context: &Context) {
        match object {
            KubeObjectType::CoreV1Node(ref o) => self.core_v1_node(o, context),
            KubeObjectType::CoreV1Pod(ref o) => self.core_v1_pod(o, context),
            KubeObjectType::CoreV1Service(ref o) => self.core_v1_service(o, context),
            KubeObjectType::AppsV1DaemonSet(ref o) => self.apps_v1_daemon_set(o, context),
            KubeObjectType::AppsV1Deployment(ref o) => self.apps_v1_deployment(o, context),
            KubeObjectType::AppsV1ReplicaSet(ref o) => self.apps_v1_replica_set(o, context),
            KubeObjectType::AppsV1StatefulSet(ref o) => self.apps_v1_stateful_set(o, context),
            KubeObjectType::PolicyV1beta1PodDisruptionBudget(ref o) => {
                self.policy_v1beta1_pod_disruption_budget(o, context)
            }
            KubeObjectType::AutoscalingV1HorizontalPodAutoscaler(ref o) => {
                self.autoscaling_v1_horizontal_pod_autoscaler(o, context)
            }
            KubeObjectType::AutoscalingV2beta1HorizontalPodAutoscaler(ref o) => {
                self.autoscaling_v2beta1_horizontal_pod_autoscaler(o, context)
            }
            KubeObjectType::AutoscalingV2beta2HorizontalPodAutoscaler(ref o) => {
                self.autoscaling_v2beta2_horizontal_pod_autoscaler(o, context)
            }
            KubeObjectType::NetworkingV1beta1Ingress(ref o) => {
                self.networking_v1beta1_ingress(o, context)
            }
            KubeObjectType::ExtensionsV1beta1Ingress(ref o) => {
                self.extensions_v1beta1_ingress(o, context)
            }
            KubeObjectType::RbacV1ClusterRole(ref o) => self.rbac_v1_cluster_role(o, context),
            KubeObjectType::RbacV1Role(ref o) => self.rbac_v1_role(o, context),
        }
    }
}

#[allow(unused)]
pub enum KubeObjectType {
    CoreV1Node(Box<k8s_openapi::api::core::v1::Node>),
    CoreV1Pod(Box<k8s_openapi::api::core::v1::Pod>),
    CoreV1Service(Box<k8s_openapi::api::core::v1::Service>),
    AppsV1DaemonSet(Box<k8s_openapi::api::apps::v1::DaemonSet>),
    AppsV1Deployment(Box<k8s_openapi::api::apps::v1::Deployment>),
    AppsV1ReplicaSet(Box<k8s_openapi::api::apps::v1::ReplicaSet>),
    AppsV1StatefulSet(Box<k8s_openapi::api::apps::v1::StatefulSet>),
    PolicyV1beta1PodDisruptionBudget(Box<k8s_openapi::api::policy::v1beta1::PodDisruptionBudget>),
    AutoscalingV1HorizontalPodAutoscaler(
        Box<k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler>,
    ),
    AutoscalingV2beta1HorizontalPodAutoscaler(
        Box<k8s_openapi::api::autoscaling::v2beta1::HorizontalPodAutoscaler>,
    ),
    AutoscalingV2beta2HorizontalPodAutoscaler(
        Box<k8s_openapi::api::autoscaling::v2beta2::HorizontalPodAutoscaler>,
    ),
    NetworkingV1beta1Ingress(Box<k8s_openapi::api::networking::v1beta1::Ingress>),
    ExtensionsV1beta1Ingress(Box<k8s_openapi::api::extensions::v1beta1::Ingress>),
    RbacV1ClusterRole(Box<k8s_openapi::api::rbac::v1::ClusterRole>),
    RbacV1Role(Box<k8s_openapi::api::rbac::v1::Role>),
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

                Ok(KubeObjectType::CoreV1Node(object))
            }

            ("core", "v1", "Pod") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::CoreV1Pod(object))
            }

            ("core", "v1", "Service") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::CoreV1Service(object))
            }

            ("apps", "v1", "DaemonSet") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::AppsV1DaemonSet(object))
            }

            ("apps", "v1", "Deployment") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::AppsV1Deployment(object))
            }

            ("apps", "v1", "ReplicaSet") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::AppsV1ReplicaSet(object))
            }

            ("apps", "v1", "StatefulSet") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::AppsV1StatefulSet(object))
            }

            ("policy", "v1beta1", "PodDisruptionBudget") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::PolicyV1beta1PodDisruptionBudget(object))
            }

            ("autoscaling", "v1", "HorizontalPodAutoscaler") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::AutoscalingV1HorizontalPodAutoscaler(object))
            }

            ("autoscaling", "v2beta1", "HorizontalPodAutoscaler") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::AutoscalingV2beta1HorizontalPodAutoscaler(
                    object,
                ))
            }

            ("autoscaling", "v2beta2", "HorizontalPodAutoscaler") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::AutoscalingV2beta2HorizontalPodAutoscaler(
                    object,
                ))
            }

            ("networking.k8s.io", "v1beta1", "Ingress") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::NetworkingV1beta1Ingress(object))
            }

            ("extensions", "v1beta1", "Ingress") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::ExtensionsV1beta1Ingress(object))
            }

            ("rbac.authorization.k8s.io", "v1", "ClusterRole") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::RbacV1ClusterRole(object))
            }

            ("rbac.authorization.k8s.io", "v1", "Role") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::RbacV1Role(object))
            }
            _ => Err(anyhow!("Could not decode the given object type")),
        }
    }

    pub fn matches_type(&self, api_version: &str, kind: &str) -> bool {
        let (ty, version) = if api_version.contains('/') {
            let mut parts = api_version.split('/');
            (parts.next().unwrap(), parts.next().unwrap())
        } else {
            ("core", api_version)
        };

        match *self {
            KubeObjectType::CoreV1Node(_) => ty == "core" && version == "v1" && kind == "Node",

            KubeObjectType::CoreV1Pod(_) => ty == "core" && version == "v1" && kind == "Pod",

            KubeObjectType::CoreV1Service(_) => {
                ty == "core" && version == "v1" && kind == "Service"
            }

            KubeObjectType::AppsV1DaemonSet(_) => {
                ty == "apps" && version == "v1" && kind == "DaemonSet"
            }

            KubeObjectType::AppsV1Deployment(_) => {
                ty == "apps" && version == "v1" && kind == "Deployment"
            }

            KubeObjectType::AppsV1ReplicaSet(_) => {
                ty == "apps" && version == "v1" && kind == "ReplicaSet"
            }

            KubeObjectType::AppsV1StatefulSet(_) => {
                ty == "apps" && version == "v1" && kind == "StatefulSet"
            }

            KubeObjectType::PolicyV1beta1PodDisruptionBudget(_) => {
                ty == "policy" && version == "v1beta1" && kind == "PodDisruptionBudget"
            }

            KubeObjectType::AutoscalingV1HorizontalPodAutoscaler(_) => {
                ty == "autoscaling" && version == "v1" && kind == "HorizontalPodAutoscaler"
            }

            KubeObjectType::AutoscalingV2beta1HorizontalPodAutoscaler(_) => {
                ty == "autoscaling" && version == "v2beta1" && kind == "HorizontalPodAutoscaler"
            }

            KubeObjectType::AutoscalingV2beta2HorizontalPodAutoscaler(_) => {
                ty == "autoscaling" && version == "v2beta2" && kind == "HorizontalPodAutoscaler"
            }

            KubeObjectType::NetworkingV1beta1Ingress(_) => {
                ty == "networking.k8s.io" && version == "v1beta1" && kind == "Ingress"
            }

            KubeObjectType::ExtensionsV1beta1Ingress(_) => {
                ty == "extensions" && version == "v1beta1" && kind == "Ingress"
            }

            KubeObjectType::RbacV1ClusterRole(_) => {
                ty == "rbac.authorization.k8s.io" && version == "v1" && kind == "ClusterRole"
            }

            KubeObjectType::RbacV1Role(_) => {
                ty == "rbac.authorization.k8s.io" && version == "v1" && kind == "Role"
            }
        }
    }

    pub fn metadata(&self) -> Option<&ObjectMeta> {
        match *self {
            KubeObjectType::CoreV1Node(ref o) => o.metadata.as_ref(),

            KubeObjectType::CoreV1Pod(ref o) => o.metadata.as_ref(),

            KubeObjectType::CoreV1Service(ref o) => o.metadata.as_ref(),

            KubeObjectType::AppsV1DaemonSet(ref o) => o.metadata.as_ref(),

            KubeObjectType::AppsV1Deployment(ref o) => o.metadata.as_ref(),

            KubeObjectType::AppsV1ReplicaSet(ref o) => o.metadata.as_ref(),

            KubeObjectType::AppsV1StatefulSet(ref o) => o.metadata.as_ref(),

            KubeObjectType::PolicyV1beta1PodDisruptionBudget(ref o) => o.metadata.as_ref(),

            KubeObjectType::AutoscalingV1HorizontalPodAutoscaler(ref o) => o.metadata.as_ref(),

            KubeObjectType::AutoscalingV2beta1HorizontalPodAutoscaler(ref o) => o.metadata.as_ref(),

            KubeObjectType::AutoscalingV2beta2HorizontalPodAutoscaler(ref o) => o.metadata.as_ref(),

            KubeObjectType::NetworkingV1beta1Ingress(ref o) => o.metadata.as_ref(),

            KubeObjectType::ExtensionsV1beta1Ingress(ref o) => o.metadata.as_ref(),

            KubeObjectType::RbacV1ClusterRole(ref o) => o.metadata.as_ref(),

            KubeObjectType::RbacV1Role(ref o) => o.metadata.as_ref(),
        }
    }
}

impl From<k8s_openapi::api::core::v1::Node> for KubeObjectType {
    fn from(o: k8s_openapi::api::core::v1::Node) -> Self {
        Self::CoreV1Node(Box::new(o))
    }
}

impl From<k8s_openapi::api::core::v1::Pod> for KubeObjectType {
    fn from(o: k8s_openapi::api::core::v1::Pod) -> Self {
        Self::CoreV1Pod(Box::new(o))
    }
}

impl From<k8s_openapi::api::core::v1::Service> for KubeObjectType {
    fn from(o: k8s_openapi::api::core::v1::Service) -> Self {
        Self::CoreV1Service(Box::new(o))
    }
}

impl From<k8s_openapi::api::apps::v1::DaemonSet> for KubeObjectType {
    fn from(o: k8s_openapi::api::apps::v1::DaemonSet) -> Self {
        Self::AppsV1DaemonSet(Box::new(o))
    }
}

impl From<k8s_openapi::api::apps::v1::Deployment> for KubeObjectType {
    fn from(o: k8s_openapi::api::apps::v1::Deployment) -> Self {
        Self::AppsV1Deployment(Box::new(o))
    }
}

impl From<k8s_openapi::api::apps::v1::ReplicaSet> for KubeObjectType {
    fn from(o: k8s_openapi::api::apps::v1::ReplicaSet) -> Self {
        Self::AppsV1ReplicaSet(Box::new(o))
    }
}

impl From<k8s_openapi::api::apps::v1::StatefulSet> for KubeObjectType {
    fn from(o: k8s_openapi::api::apps::v1::StatefulSet) -> Self {
        Self::AppsV1StatefulSet(Box::new(o))
    }
}

impl From<k8s_openapi::api::policy::v1beta1::PodDisruptionBudget> for KubeObjectType {
    fn from(o: k8s_openapi::api::policy::v1beta1::PodDisruptionBudget) -> Self {
        Self::PolicyV1beta1PodDisruptionBudget(Box::new(o))
    }
}

impl From<k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler> for KubeObjectType {
    fn from(o: k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler) -> Self {
        Self::AutoscalingV1HorizontalPodAutoscaler(Box::new(o))
    }
}

impl From<k8s_openapi::api::autoscaling::v2beta1::HorizontalPodAutoscaler> for KubeObjectType {
    fn from(o: k8s_openapi::api::autoscaling::v2beta1::HorizontalPodAutoscaler) -> Self {
        Self::AutoscalingV2beta1HorizontalPodAutoscaler(Box::new(o))
    }
}

impl From<k8s_openapi::api::autoscaling::v2beta2::HorizontalPodAutoscaler> for KubeObjectType {
    fn from(o: k8s_openapi::api::autoscaling::v2beta2::HorizontalPodAutoscaler) -> Self {
        Self::AutoscalingV2beta2HorizontalPodAutoscaler(Box::new(o))
    }
}

impl From<k8s_openapi::api::networking::v1beta1::Ingress> for KubeObjectType {
    fn from(o: k8s_openapi::api::networking::v1beta1::Ingress) -> Self {
        Self::NetworkingV1beta1Ingress(Box::new(o))
    }
}

impl From<k8s_openapi::api::extensions::v1beta1::Ingress> for KubeObjectType {
    fn from(o: k8s_openapi::api::extensions::v1beta1::Ingress) -> Self {
        Self::ExtensionsV1beta1Ingress(Box::new(o))
    }
}

impl From<k8s_openapi::api::rbac::v1::ClusterRole> for KubeObjectType {
    fn from(o: k8s_openapi::api::rbac::v1::ClusterRole) -> Self {
        Self::RbacV1ClusterRole(Box::new(o))
    }
}

impl From<k8s_openapi::api::rbac::v1::Role> for KubeObjectType {
    fn from(o: k8s_openapi::api::rbac::v1::Role) -> Self {
        Self::RbacV1Role(Box::new(o))
    }
}
