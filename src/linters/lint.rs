use k8s_openapi::api::core;
use k8s_openapi::api::policy;
use k8s_openapi::api::apps;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

pub trait Lint {
	fn v1_namespace(&self, _namespace: &core::v1::NamespaceSpec, metadata: &ObjectMeta) -> Option<Vec<crate::reporting::Finding>> { None }
	fn v1_node(&self, _node: &core::v1::NodeSpec, metadata: &ObjectMeta) -> Option<Vec<crate::reporting::Finding>> { None }
	fn v1_pod(&self, _pod: &core::v1::PodSpec, metadata: &ObjectMeta) -> Option<Vec<crate::reporting::Finding>> { None }
	fn v1_replication_controller(&self, _replication_controller: &core::v1::ReplicationControllerSpec, metadata: &ObjectMeta) -> Option<Vec<crate::reporting::Finding>> { None }
	fn v1_service(&self, _service: &core::v1::ServiceSpec, metadata: &ObjectMeta) -> Option<Vec<crate::reporting::Finding>> { None }
	fn v1_daemon_set(&self, _daemon_set: &apps::v1::DaemonSetSpec, metadata: &ObjectMeta) -> Option<Vec<crate::reporting::Finding>> { None }
	fn v1_deployment(&self, _deployment: &apps::v1::DeploymentSpec, metadata: &ObjectMeta) -> Option<Vec<crate::reporting::Finding>> { None }
	fn v1_replica_set(&self, _replica_set: &apps::v1::ReplicaSetSpec, metadata: &ObjectMeta) -> Option<Vec<crate::reporting::Finding>> { None }
	fn v1_stateful_set(&self, _stateful_set: &apps::v1::StatefulSetSpec, metadata: &ObjectMeta) -> Option<Vec<crate::reporting::Finding>> { None }
	fn v1beta1_pod_disruption_budget(&self, _pod_disruption_budget: &policy::v1beta1::PodDisruptionBudgetSpec, metadata: &ObjectMeta) -> Option<Vec<crate::reporting::Finding>> { None }

}