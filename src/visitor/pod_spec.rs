use crate::linters::KubeObjectType;
use k8s_openapi::api::core::v1::PodSpec;
use kube::api::{KubeObject, ObjectMeta};

pub(crate) trait PodSpecVisitor {
    fn visit_pod_spec(&mut self, pod_spec: &PodSpec, meta: &ObjectMeta);
}

pub(crate) fn pod_spec_visit<V: PodSpecVisitor>(object: &KubeObjectType, visitor: &mut V) {
    match object {
        KubeObjectType::V1Pod(pod) => {
            visitor.visit_pod_spec(&pod.spec, &pod.meta());
        }
        KubeObjectType::V1Deployment(object) => {
            if let Some(pod_spec) = &object.spec.template.spec {
                visitor.visit_pod_spec(pod_spec, object.meta());
            }
        }
        KubeObjectType::V1DaemonSet(object) => {
            if let Some(pod_spec) = &object.spec.template.spec {
                visitor.visit_pod_spec(pod_spec, object.meta());
            }
        }
        KubeObjectType::V1ReplicaSet(object) => {
            if let Some(template) = &object.spec.template {
                if let Some(ref pod_spec) = template.spec {
                    visitor.visit_pod_spec(pod_spec, object.meta());
                }
            }
        }
        KubeObjectType::V1StatefulSet(object) => {
            if let Some(pod_spec) = &object.spec.template.spec {
                visitor.visit_pod_spec(pod_spec, object.meta());
            }
        }
        // Those objects do not contain any podspec
        KubeObjectType::V1Node(_)
        | KubeObjectType::V1Service(_)
        | KubeObjectType::V1beta1PodDisruptionBudget(_)
        | KubeObjectType::V1HorizontalPodAutoscaler(_) => {}
    }
}
