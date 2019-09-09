use k8s_openapi::api::apps;
use k8s_openapi::api::core;
use kube::api::Object;
use crate::linters::LintSpec;

pub trait Lint {
	fn v1_node(&self, _node: &Object<core::v1::NodeSpec, core::v1::NodeStatus>) -> Vec<crate::reporting::Finding> { Vec::new() }
	fn v1_pod(&self, _pod: &Object<core::v1::PodSpec, core::v1::PodStatus>) -> Vec<crate::reporting::Finding> { Vec::new() }
	fn v1_service(&self, _service: &Object<core::v1::ServiceSpec, core::v1::ServiceStatus>) -> Vec<crate::reporting::Finding> { Vec::new() }
	fn v1_deployment(&self, _deployment: &Object<apps::v1::DeploymentSpec, apps::v1::DeploymentStatus>) -> Vec<crate::reporting::Finding> { Vec::new() }

    fn spec(&self) -> LintSpec;
}

pub enum KubeObjectType {
	V1Node(Object<core::v1::NodeSpec, core::v1::NodeStatus>), 
	V1Pod(Object<core::v1::PodSpec, core::v1::PodStatus>), 
	V1Service(Object<core::v1::ServiceSpec, core::v1::ServiceStatus>), 
	V1Deployment(Object<apps::v1::DeploymentSpec, apps::v1::DeploymentStatus>), 

    #[doc(hidden)]
    __Nonexhaustive,
}