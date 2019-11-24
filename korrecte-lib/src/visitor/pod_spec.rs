use crate::f;
use crate::linters::KubeObjectType;
use k8s_openapi::api::core::v1::PodSpec;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

pub(crate) trait PodSpecVisitor {
    fn visit_pod_spec(&mut self, pod_spec: &PodSpec, meta: Option<&ObjectMeta>);
}

pub(crate) fn pod_spec_visit<V: PodSpecVisitor>(object: &KubeObjectType, visitor: &mut V) {
    match object {
        KubeObjectType::V1Pod(pod) => {
            visitor.visit_pod_spec(&pod.spec.as_ref().unwrap(), pod.metadata.as_ref());
        }
        KubeObjectType::V1Deployment(object) => {
            let maybe_pod_spec = object
                .spec
                .as_ref()
                .map(|s| &s.template)
                .and_then(|template| template.spec.as_ref());

            if let Some(pod_spec) = maybe_pod_spec {
                visitor.visit_pod_spec(pod_spec, object.metadata.as_ref());
            }
        }
        KubeObjectType::V1DaemonSet(object) => {
            let maybe_pod_spec = object
                .spec
                .as_ref()
                .map(|s| &s.template)
                .and_then(|template| template.spec.as_ref());

            if let Some(pod_spec) = maybe_pod_spec {
                visitor.visit_pod_spec(pod_spec, object.metadata.as_ref());
            }
        }
        KubeObjectType::V1ReplicaSet(object) => {
            let maybe_pod_spec = f!(object.spec, template, spec);

            if let Some(pod_spec) = maybe_pod_spec {
                visitor.visit_pod_spec(pod_spec, object.metadata.as_ref());
            }
        }
        KubeObjectType::V1StatefulSet(object) => {
            let maybe_pod_spec = object
                .spec
                .as_ref()
                .map(|s| &s.template)
                .and_then(|template| template.spec.as_ref());

            if let Some(pod_spec) = maybe_pod_spec {
                visitor.visit_pod_spec(pod_spec, object.metadata.as_ref());
            }
        }
        // Those objects do not contain any podspec
        KubeObjectType::V1Node(_)
        | KubeObjectType::V1Service(_)
        | KubeObjectType::V1beta1PodDisruptionBudget(_)
        | KubeObjectType::V1HorizontalPodAutoscaler(_)
        | KubeObjectType::V1beta1Ingress(_) => {}
    }
}
