use crate::linters::evaluator::Context;
use anyhow::{anyhow, Result};

pub trait Lint {
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
    fn extensions_v1beta1_network_policy(
        &self,
        _network_policy: &k8s_openapi::api::extensions::v1beta1::NetworkPolicy,
        _context: &Context,
    ) {
    }
    fn extensions_v1beta1_pod_security_policy(
        &self,
        _pod_security_policy: &k8s_openapi::api::extensions::v1beta1::PodSecurityPolicy,
        _context: &Context,
    ) {
    }
    fn extensions_v1beta1_daemon_set(
        &self,
        _daemon_set: &k8s_openapi::api::extensions::v1beta1::DaemonSet,
        _context: &Context,
    ) {
    }
    fn extensions_v1beta1_deployment(
        &self,
        _deployment: &k8s_openapi::api::extensions::v1beta1::Deployment,
        _context: &Context,
    ) {
    }
    fn extensions_v1beta1_replica_set(
        &self,
        _replica_set: &k8s_openapi::api::extensions::v1beta1::ReplicaSet,
        _context: &Context,
    ) {
    }
    fn apps_v1beta2_daemon_set(
        &self,
        _daemon_set: &k8s_openapi::api::apps::v1beta2::DaemonSet,
        _context: &Context,
    ) {
    }
    fn apps_v1beta2_deployment(
        &self,
        _deployment: &k8s_openapi::api::apps::v1beta2::Deployment,
        _context: &Context,
    ) {
    }
    fn apps_v1beta2_replica_set(
        &self,
        _replica_set: &k8s_openapi::api::apps::v1beta2::ReplicaSet,
        _context: &Context,
    ) {
    }

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
            KubeObjectType::NetworkingV1beta1Ingress(ref o) => {
                self.networking_v1beta1_ingress(o, context)
            }
            KubeObjectType::ExtensionsV1beta1Ingress(ref o) => {
                self.extensions_v1beta1_ingress(o, context)
            }
            KubeObjectType::RbacV1ClusterRole(ref o) => self.rbac_v1_cluster_role(o, context),
            KubeObjectType::RbacV1Role(ref o) => self.rbac_v1_role(o, context),
            KubeObjectType::ExtensionsV1beta1NetworkPolicy(ref o) => {
                self.extensions_v1beta1_network_policy(o, context)
            }
            KubeObjectType::ExtensionsV1beta1PodSecurityPolicy(ref o) => {
                self.extensions_v1beta1_pod_security_policy(o, context)
            }
            KubeObjectType::ExtensionsV1beta1DaemonSet(ref o) => {
                self.extensions_v1beta1_daemon_set(o, context)
            }
            KubeObjectType::ExtensionsV1beta1Deployment(ref o) => {
                self.extensions_v1beta1_deployment(o, context)
            }
            KubeObjectType::ExtensionsV1beta1ReplicaSet(ref o) => {
                self.extensions_v1beta1_replica_set(o, context)
            }
            KubeObjectType::AppsV1beta2DaemonSet(ref o) => self.apps_v1beta2_daemon_set(o, context),
            KubeObjectType::AppsV1beta2Deployment(ref o) => {
                self.apps_v1beta2_deployment(o, context)
            }
            KubeObjectType::AppsV1beta2ReplicaSet(ref o) => {
                self.apps_v1beta2_replica_set(o, context)
            }
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
    NetworkingV1beta1Ingress(Box<k8s_openapi::api::networking::v1beta1::Ingress>),
    ExtensionsV1beta1Ingress(Box<k8s_openapi::api::extensions::v1beta1::Ingress>),
    RbacV1ClusterRole(Box<k8s_openapi::api::rbac::v1::ClusterRole>),
    RbacV1Role(Box<k8s_openapi::api::rbac::v1::Role>),
    ExtensionsV1beta1NetworkPolicy(Box<k8s_openapi::api::extensions::v1beta1::NetworkPolicy>),
    ExtensionsV1beta1PodSecurityPolicy(
        Box<k8s_openapi::api::extensions::v1beta1::PodSecurityPolicy>,
    ),
    ExtensionsV1beta1DaemonSet(Box<k8s_openapi::api::extensions::v1beta1::DaemonSet>),
    ExtensionsV1beta1Deployment(Box<k8s_openapi::api::extensions::v1beta1::Deployment>),
    ExtensionsV1beta1ReplicaSet(Box<k8s_openapi::api::extensions::v1beta1::ReplicaSet>),
    AppsV1beta2DaemonSet(Box<k8s_openapi::api::apps::v1beta2::DaemonSet>),
    AppsV1beta2Deployment(Box<k8s_openapi::api::apps::v1beta2::Deployment>),
    AppsV1beta2ReplicaSet(Box<k8s_openapi::api::apps::v1beta2::ReplicaSet>),
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

            ("extensions", "v1beta1", "NetworkPolicy") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::ExtensionsV1beta1NetworkPolicy(object))
            }

            ("extensions", "v1beta1", "PodSecurityPolicy") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::ExtensionsV1beta1PodSecurityPolicy(object))
            }

            ("extensions", "v1beta1", "DaemonSet") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::ExtensionsV1beta1DaemonSet(object))
            }

            ("extensions", "v1beta1", "Deployment") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::ExtensionsV1beta1Deployment(object))
            }

            ("extensions", "v1beta1", "ReplicaSet") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::ExtensionsV1beta1ReplicaSet(object))
            }

            ("apps", "v1beta2", "DaemonSet") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::AppsV1beta2DaemonSet(object))
            }

            ("apps", "v1beta2", "Deployment") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::AppsV1beta2Deployment(object))
            }

            ("apps", "v1beta2", "ReplicaSet") => {
                let object = serde_yaml::from_str(yaml)?;

                Ok(KubeObjectType::AppsV1beta2ReplicaSet(object))
            }
            _ => Err(anyhow!("Could not decode the given object type")),
        }
    }
}
