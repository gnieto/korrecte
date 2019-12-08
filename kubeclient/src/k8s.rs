use crate::kubeclient::Object;
use k8s_openapi::k8s_if_ge_1_9;

impl Object for k8s_openapi::api::core::v1::Pod {
    fn prefix() -> &'static str {
        "api"
    }

    fn name() -> &'static str { "pods" }

    fn is_namespaced() -> bool { true }
}

impl Object for k8s_openapi::api::core::v1::Node {
    fn prefix() -> &'static str {
        "api"
    }

    fn name() -> &'static str { "nodes" }

    fn is_namespaced() -> bool { false }
}

impl Object for k8s_openapi::api::core::v1::Service {
    fn prefix() -> &'static str {
        "api"
    }

    fn name() -> &'static str { "services" }

    fn is_namespaced() -> bool { true }
}

k8s_if_ge_1_9! {
    impl Object for k8s_openapi::api::apps::v1::DaemonSet {
        fn prefix() -> &'static str {
            "apis"
        }

        fn name() -> &'static str { "daemonsets" }

        fn is_namespaced() -> bool { false }
    }
}
k8s_if_ge_1_9! {
    impl Object for k8s_openapi::api::apps::v1::Deployment {
        fn prefix() -> &'static str {
            "apis"
        }

        fn name() -> &'static str {
            "deployments"
        }

        fn is_namespaced() -> bool {
            true
        }
    }
}

k8s_if_ge_1_9! {
    impl Object for k8s_openapi::api::apps::v1::ReplicaSet {
        fn prefix() -> &'static str {
            "apis"
        }

        fn name() -> &'static str {
            "replicasets"
        }

        fn is_namespaced() -> bool {
            true
        }
    }
}

k8s_if_ge_1_9! {
    impl Object for k8s_openapi::api::apps::v1::StatefulSet {
        fn prefix() -> &'static str {
            "apis"
        }

        fn name() -> &'static str {
            "statefulsets"
        }

        fn is_namespaced() -> bool {
            true
        }
    }
}

impl Object for k8s_openapi::api::policy::v1beta1::PodDisruptionBudget {
    fn prefix() -> &'static str {
        "apis"
    }

    fn name() -> &'static str {
        "poddisruptionbudgets"
    }

    fn is_namespaced() -> bool {
        true
    }
}

impl Object for k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler {
    fn prefix() -> &'static str {
        "apis"
    }

    fn name() -> &'static str {
        "horizontalpodautoscalers"
    }

    fn is_namespaced() -> bool {
        true
    }
}

k8s_if_ge_1_9! {
    impl Object for k8s_openapi::api::networking::v1beta1::Ingress {
        fn prefix() -> &'static str {
            "apis"
        }

        fn name() -> &'static str {
            "ingresses"
        }

        fn is_namespaced() -> bool {
            true
        }
    }
}

impl Object for k8s_openapi::api::extensions::v1beta1::Ingress {
    fn prefix() -> &'static str {
        "apis"
    }

    fn name() -> &'static str {
        "ingresses"
    }

    fn is_namespaced() -> bool {
        true
    }
}

impl Object for k8s_openapi::api::rbac::v1::Role {
    fn prefix() -> &'static str {
        "apis"
    }

    fn name() -> &'static str {
        "roles"
    }

    fn is_namespaced() -> bool {
        true
    }
}

impl Object for k8s_openapi::api::rbac::v1::ClusterRole {
    fn prefix() -> &'static str {
        "apis"
    }

    fn name() -> &'static str {
        "clusterroles"
    }

    fn is_namespaced() -> bool {
        true
    }
}
