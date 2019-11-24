use crate::kubeclient::Object;

impl Object for k8s_openapi::api::core::v1::Pod {
    fn prefix() -> &'static str {
        "api"
    }
}

impl Object for k8s_openapi::api::core::v1::Node {
    fn prefix() -> &'static str {
        "api"
    }
}

impl Object for k8s_openapi::api::core::v1::Service {
    fn prefix() -> &'static str {
        "api"
    }
}

impl Object for k8s_openapi::api::apps::v1::DaemonSet {
    fn prefix() -> &'static str {
        "apis"
    }
}

impl Object for k8s_openapi::api::apps::v1::Deployment {
    fn prefix() -> &'static str {
        "apis"
    }
}

impl Object for k8s_openapi::api::apps::v1::ReplicaSet {
    fn prefix() -> &'static str {
        "apis"
    }
}

impl Object for k8s_openapi::api::apps::v1::StatefulSet {
    fn prefix() -> &'static str {
        "apis"
    }
}

impl Object for k8s_openapi::api::policy::v1beta1::PodDisruptionBudget {
    fn prefix() -> &'static str {
        "apis"
    }
}

impl Object for k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler {
    fn prefix() -> &'static str {
        "apis"
    }
}

impl Object for k8s_openapi::api::networking::v1beta1::Ingress {
    fn prefix() -> &'static str {
        "apis"
    }
    fn resource() -> String {
        "ingresses".to_string()
    }
}

impl Object for k8s_openapi::api::rbac::v1::Role {
    fn prefix() -> &'static str {
        "apis"
    }
}

impl Object for k8s_openapi::api::rbac::v1::ClusterRole {
    fn prefix() -> &'static str {
        "apis"
    }
}
