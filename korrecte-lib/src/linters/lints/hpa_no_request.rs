use crate::linters::evaluator::Context;
use crate::linters::{KubeObjectType, Lint};
use crate::reporting::Finding;
use crate::visitor::{pod_spec_visit, PodSpecVisitor};
use crate::{f, m};
use k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler;
use k8s_openapi::api::core::v1::PodSpec;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

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
        let matching_controller = TargetExtractor::extract(
            &context,
            reference.api_version.as_ref(),
            &reference.kind,
            &reference.name,
        );

        if let Some(obj) = matching_controller {
            let mut v = MissingCpuRequestVisitor {
                any_container_without_cpu: false,
            };
            pod_spec_visit(obj, &mut v);

            if v.any_container_without_cpu {
                context
                    .reporter
                    .report(Finding::new(LINT_NAME, hpa.metadata.clone()));
            }
        }
    }
}

struct TargetExtractor;

impl TargetExtractor {
    fn extract<'a>(
        context: &'a Context,
        api_version: Option<&'a String>,
        kind: &'a str,
        name: &'a str,
    ) -> Option<&'a KubeObjectType> {
        context.repository.iter().find(|k| {
            let current_name = f!(k.metadata(), name).map(|meta| meta.as_str());

            k.matches_type(api_version.unwrap(), kind) && current_name == Some(name)
        })
    }
}

struct MissingCpuRequestVisitor {
    any_container_without_cpu: bool,
}

impl PodSpecVisitor for MissingCpuRequestVisitor {
    fn visit_pod_spec(&mut self, pod_spec: &PodSpec, _: &ObjectMeta, _: Option<&ObjectMeta>) {
        self.any_container_without_cpu = pod_spec.containers.iter().any(|c| {
            let requests = f!(c.resources, requests);
            let has_cpu_request = requests.map(|r| r.contains_key("cpu")).unwrap_or(false);
            !has_cpu_request
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{analyze_file, filter_findings_by};
    use std::path::Path;

    #[test]
    fn it_detects_missconfigured_hpa() {
        let findings = analyze_file(Path::new("../tests/hpa_no_request.yaml"));
        let findings = filter_findings_by(findings, super::LINT_NAME);

        assert_eq!(1, findings.len());
        assert_eq!("hpa-no-cpu-request", findings[0].name());
    }
}
