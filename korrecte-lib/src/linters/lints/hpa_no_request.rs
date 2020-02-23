use crate::linters::{Lint, KubeObjectType};
use crate::linters::evaluator::Context;
use k8s_openapi::api::autoscaling::v1::{HorizontalPodAutoscaler, CrossVersionObjectReference};
use crate::m;

#[derive(Default)]
pub(crate) struct HpaNoRequest;

const LINT_NAME: &str = "hpa_no_request";

impl Lint for HpaNoRequest {
    fn name(&self) -> &str {
        LINT_NAME
    }

    fn autoscaling_v1_horizontal_pod_autoscaler(
        &self,
        hpa: &HorizontalPodAutoscaler,
        context: &Context,
    ) {
        let cross_reference = m!(hpa.spec, scale_target_ref);
        if cross_reference.is_none() {
            return;
        }
        let reference = cross_reference.unwrap();
        let matching_controller = TargetExtractor::extract(&context, reference.api_version.as_ref(), &reference.kind, &reference.name);


    }
}

struct TargetExtractor;

impl TargetExtractor {
    fn extract(context: &Context, api_version: Option<&String>, kind: &str, name: &str) -> Option<KubeObjectType> {
        context.repository.iter()
            .filter(|k| {
                match k {

                }
            })
    }
}