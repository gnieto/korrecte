use crate::linters::evaluator::Context;
use crate::linters::{KubeObjectType, Lint};
use crate::reporting::Finding;
use crate::visitor::{pod_spec_visit, PodSpecVisitor};
use crate::{f, m};
use k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler;
use k8s_openapi::api::autoscaling::v2beta1::HorizontalPodAutoscaler as HpaBeta2;
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

        if f!(hpa.spec, target_cpu_utilization_percentage).is_none() {
            return;
        }

        let reference = cross_reference.unwrap();
        let matching_controller = TargetExtractor::extract(
            &context,
            reference.api_version.as_ref(),
            &reference.kind,
            &reference.name,
        );

        if matching_controller.is_none() {
            return;
        }

        let mut v = MissingCpuRequestVisitor {
            any_container_without_cpu: false,
            any_container_without_mem: false,
        };
        pod_spec_visit(matching_controller.unwrap(), &mut v);

        if v.any_container_without_cpu {
            context
                .reporter
                .report(Finding::new(LINT_NAME, hpa.metadata.clone()));
        }
    }

    fn autoscaling_v2beta1_horizontal_pod_autoscaler(&self, hpa: &HpaBeta2, context: &Context) {
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

        if matching_controller.is_none() {
            return;
        }

        let mut v = MissingCpuRequestVisitor {
            any_container_without_cpu: false,
            any_container_without_mem: false,
        };
        pod_spec_visit(matching_controller.unwrap(), &mut v);

        f!(hpa.spec, metrics).map(|metrics| {
            for m in metrics {
                if m.type_ != "Resource" {
                    break;
                }

                let metric_name = m!(m.resource, name).map(|n| n.as_str());
                if metric_name == Some("cpu") && v.any_container_without_cpu
                    || metric_name == Some("memory") && v.any_container_without_mem
                {
                    context
                        .reporter
                        .report(Finding::new(LINT_NAME, hpa.metadata.clone()));
                }
            }
        });
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

            match api_version {
                Some(api_version) => {
                    k.matches_type(api_version, kind) && current_name == Some(name)
                }
                None => false,
            }
        })
    }
}

struct MissingCpuRequestVisitor {
    any_container_without_cpu: bool,
    any_container_without_mem: bool,
}

impl PodSpecVisitor for MissingCpuRequestVisitor {
    fn visit_pod_spec(&mut self, pod_spec: &PodSpec, _: &ObjectMeta, _: Option<&ObjectMeta>) {
        self.any_container_without_cpu = pod_spec.containers.iter().any(|c| {
            let requests = f!(c.resources, requests);
            let has_cpu_request = requests.map(|r| r.contains_key("cpu")).unwrap_or(false);
            !has_cpu_request
        });

        self.any_container_without_mem = pod_spec.containers.iter().any(|c| {
            let requests = f!(c.resources, requests);
            let has_mem_request = requests.map(|r| r.contains_key("memory")).unwrap_or(false);
            !has_mem_request
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

        assert_eq!(3, findings.len());
        assert_eq!("hpa-no-cpu-request", findings[0].name());
        assert_eq!("hpa-no-cpu-request-v2-cpu", findings[1].name());
        assert_eq!("hpa-no-cpu-request-v2-mem", findings[2].name());
    }
}
