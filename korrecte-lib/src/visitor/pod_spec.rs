use crate::f;
use crate::linters::evaluator::Context;
use crate::linters::KubeObjectType;
use k8s_openapi::api::core::v1::PodSpec;
use k8s_openapi::api::core::v1::PodTemplateSpec;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

pub(crate) trait PodSpecVisitor {
    fn visit_pod_spec(
        &mut self,
        pod_spec: &PodSpec,
        pod_meta: &ObjectMeta,
        meta: Option<&ObjectMeta>,
    );
}

pub(crate) fn visit_all_pod_specs<V: PodSpecVisitor>(context: &Context, visitor: &mut V) {
    for objects in context.repository.iter() {
        pod_spec_visit(&objects, visitor);
    }
}

pub(crate) fn pod_spec_visit<V: PodSpecVisitor>(object: &KubeObjectType, visitor: &mut V) {
    match object {
        KubeObjectType::V1Pod(pod) => {
            let meta = &pod.metadata.as_ref().unwrap();
            visitor.visit_pod_spec(&pod.spec.as_ref().unwrap(), meta, pod.metadata.as_ref());
        }
        KubeObjectType::V1Deployment(object) => {
            let maybe_template = object.spec.as_ref().map(|s| &s.template);

            visit_pod_template(maybe_template, object.metadata.as_ref(), visitor)
        }
        KubeObjectType::V1DaemonSet(object) => {
            let maybe_template = object.spec.as_ref().map(|s| &s.template);

            visit_pod_template(maybe_template, object.metadata.as_ref(), visitor)
        }
        KubeObjectType::V1ReplicaSet(object) => {
            let maybe_template = f!(object.spec, template);

            visit_pod_template(maybe_template, object.metadata.as_ref(), visitor)
        }
        KubeObjectType::V1StatefulSet(object) => {
            let maybe_template = object.spec.as_ref().map(|s| &s.template);

            visit_pod_template(maybe_template, object.metadata.as_ref(), visitor)
        }
        // Those objects do not contain any podspec
        _ => {}
    }
}

fn visit_pod_template<V: PodSpecVisitor>(
    template: Option<&PodTemplateSpec>,
    object_meta: Option<&ObjectMeta>,
    visitor: &mut V,
) {
    if let Some(template) = template {
        if let (Some(ref pod_spec), Some(ref pod_meta)) =
            (template.spec.as_ref(), template.metadata.as_ref())
        {
            visitor.visit_pod_spec(pod_spec, pod_meta, object_meta);
        }
    }
}
