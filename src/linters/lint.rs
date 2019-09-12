use crate::error::KorrecteError;
use crate::linters::LintSpec;
use k8s_openapi::api::apps;
use k8s_openapi::api::autoscaling;
use k8s_openapi::api::core;
use k8s_openapi::api::policy;
use kube::api::Object;

pub trait Lint {
    fn v1_node(
        &self,
        _node: &Object<core::v1::NodeSpec, core::v1::NodeStatus>,
    ) -> Vec<crate::reporting::Finding> {
        Vec::new()
    }
    fn v1_pod(
        &self,
        _pod: &Object<core::v1::PodSpec, core::v1::PodStatus>,
    ) -> Vec<crate::reporting::Finding> {
        Vec::new()
    }
    fn v1_service(
        &self,
        _service: &Object<core::v1::ServiceSpec, core::v1::ServiceStatus>,
    ) -> Vec<crate::reporting::Finding> {
        Vec::new()
    }
    fn v1_daemon_set(
        &self,
        _daemon_set: &Object<apps::v1::DaemonSetSpec, apps::v1::DaemonSetStatus>,
    ) -> Vec<crate::reporting::Finding> {
        Vec::new()
    }
    fn v1_deployment(
        &self,
        _deployment: &Object<apps::v1::DeploymentSpec, apps::v1::DeploymentStatus>,
    ) -> Vec<crate::reporting::Finding> {
        Vec::new()
    }
    fn v1_replica_set(
        &self,
        _replica_set: &Object<apps::v1::ReplicaSetSpec, apps::v1::ReplicaSetStatus>,
    ) -> Vec<crate::reporting::Finding> {
        Vec::new()
    }
    fn v1_stateful_set(
        &self,
        _stateful_set: &Object<apps::v1::StatefulSetSpec, apps::v1::StatefulSetStatus>,
    ) -> Vec<crate::reporting::Finding> {
        Vec::new()
    }
    fn v1beta1_pod_disruption_budget(
        &self,
        _pod_disruption_budget: &Object<
            policy::v1beta1::PodDisruptionBudgetSpec,
            policy::v1beta1::PodDisruptionBudgetStatus,
        >,
    ) -> Vec<crate::reporting::Finding> {
        Vec::new()
    }
    fn v1_horizontal_pod_autoscaler(
        &self,
        _horizontal_pod_autoscaler: &Object<
            autoscaling::v1::HorizontalPodAutoscalerSpec,
            autoscaling::v1::HorizontalPodAutoscalerStatus,
        >,
    ) -> Vec<crate::reporting::Finding> {
        Vec::new()
    }

    fn object(&self, object: &KubeObjectType) -> Vec<crate::reporting::Finding> {
        match object {
            KubeObjectType::V1Node(ref o) => self.v1_node(o),
            KubeObjectType::V1Pod(ref o) => self.v1_pod(o),
            KubeObjectType::V1Service(ref o) => self.v1_service(o),
            KubeObjectType::V1DaemonSet(ref o) => self.v1_daemon_set(o),
            KubeObjectType::V1Deployment(ref o) => self.v1_deployment(o),
            KubeObjectType::V1ReplicaSet(ref o) => self.v1_replica_set(o),
            KubeObjectType::V1StatefulSet(ref o) => self.v1_stateful_set(o),
            KubeObjectType::V1beta1PodDisruptionBudget(ref o) => {
                self.v1beta1_pod_disruption_budget(o)
            }
            KubeObjectType::V1HorizontalPodAutoscaler(ref o) => {
                self.v1_horizontal_pod_autoscaler(o)
            }
            _ => Vec::new(),
        }
    }

    fn spec(&self) -> LintSpec;
}

#[allow(unused)]
pub enum KubeObjectType {
    V1Node(Box<Object<core::v1::NodeSpec, core::v1::NodeStatus>>),
    V1Pod(Box<Object<core::v1::PodSpec, core::v1::PodStatus>>),
    V1Service(Box<Object<core::v1::ServiceSpec, core::v1::ServiceStatus>>),
    V1DaemonSet(Box<Object<apps::v1::DaemonSetSpec, apps::v1::DaemonSetStatus>>),
    V1Deployment(Box<Object<apps::v1::DeploymentSpec, apps::v1::DeploymentStatus>>),
    V1ReplicaSet(Box<Object<apps::v1::ReplicaSetSpec, apps::v1::ReplicaSetStatus>>),
    V1StatefulSet(Box<Object<apps::v1::StatefulSetSpec, apps::v1::StatefulSetStatus>>),
    V1beta1PodDisruptionBudget(
        Box<
            Object<
                policy::v1beta1::PodDisruptionBudgetSpec,
                policy::v1beta1::PodDisruptionBudgetStatus,
            >,
        >,
    ),
    V1HorizontalPodAutoscaler(
        Box<
            Object<
                autoscaling::v1::HorizontalPodAutoscalerSpec,
                autoscaling::v1::HorizontalPodAutoscalerStatus,
            >,
        >,
    ),

    #[doc(hidden)]
    __Nonexhaustive,
}

impl KubeObjectType {
    pub fn from_yaml(
        yaml: &str,
        api_version: &str,
        kind: &str,
    ) -> Result<KubeObjectType, KorrecteError> {
        let (ty, version) = if api_version.contains('/') {
            let mut parts = api_version.split('/');
            (parts.next().unwrap(), parts.next().unwrap())
        } else {
            ("core", api_version)
        };

        match (ty, version, kind) {
            ("core", "v1", "Node") => {
                let object =
                    serde_yaml::from_str(yaml).map_err(|_| KorrecteError::FailedToLoadYamlFile)?;

                Ok(KubeObjectType::V1Node(object))
            }

            ("core", "v1", "Pod") => {
                let object =
                    serde_yaml::from_str(yaml).map_err(|_| KorrecteError::FailedToLoadYamlFile)?;

                Ok(KubeObjectType::V1Pod(object))
            }

            ("core", "v1", "Service") => {
                let object =
                    serde_yaml::from_str(yaml).map_err(|_| KorrecteError::FailedToLoadYamlFile)?;

                Ok(KubeObjectType::V1Service(object))
            }

            ("apps", "v1", "DaemonSet") => {
                let object =
                    serde_yaml::from_str(yaml).map_err(|_| KorrecteError::FailedToLoadYamlFile)?;

                Ok(KubeObjectType::V1DaemonSet(object))
            }

            ("apps", "v1", "Deployment") => {
                let object =
                    serde_yaml::from_str(yaml).map_err(|_| KorrecteError::FailedToLoadYamlFile)?;

                Ok(KubeObjectType::V1Deployment(object))
            }

            ("apps", "v1", "ReplicaSet") => {
                let object =
                    serde_yaml::from_str(yaml).map_err(|_| KorrecteError::FailedToLoadYamlFile)?;

                Ok(KubeObjectType::V1ReplicaSet(object))
            }

            ("apps", "v1", "StatefulSet") => {
                let object =
                    serde_yaml::from_str(yaml).map_err(|_| KorrecteError::FailedToLoadYamlFile)?;

                Ok(KubeObjectType::V1StatefulSet(object))
            }

            ("policy", "v1beta1", "PodDisruptionBudget") => {
                let object =
                    serde_yaml::from_str(yaml).map_err(|_| KorrecteError::FailedToLoadYamlFile)?;

                Ok(KubeObjectType::V1beta1PodDisruptionBudget(object))
            }

            ("autoscaling", "v1", "HorizontalPodAutoscaler") => {
                let object =
                    serde_yaml::from_str(yaml).map_err(|_| KorrecteError::FailedToLoadYamlFile)?;

                Ok(KubeObjectType::V1HorizontalPodAutoscaler(object))
            }
            _ => Err(KorrecteError::YamlDecodeError {
                ty: ty.into(),
                version: version.into(),
                kind: kind.into(),
            }),
        }
    }
}
