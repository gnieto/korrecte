use crate::linters::Lint;

use crate::f;
use crate::kube::service::FindFistMatchingPodSpec;
use crate::linters::evaluator::Context;
use crate::reporting::Finding;
use crate::visitor::visit_all_pod_specs;
use k8s_openapi::api::core::v1::PodSpec;
use k8s_openapi::api::core::v1::Service;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use std::collections::BTreeMap;

const LINT_NAME: &str = "service_target_port";

pub(crate) struct ServiceTargetPort;

impl Lint for ServiceTargetPort {
    fn name(&self) -> &str {
        LINT_NAME
    }

    fn core_v1_service(&self, service: &Service, context: &Context) {
        if !Self::has_numeric_port(service) {
            return;
        }
        let selectors: BTreeMap<String, String> =
            f!(service.spec, selector).cloned().unwrap_or_default();

        let mut visitor = FindFistMatchingPodSpec::new(&selectors);
        visit_all_pod_specs(context, &mut visitor);

        if let Some(pod_spec) = visitor.first_matching_pod_spec() {
            Self::check_pod_spec_contains_numeric_port(service, pod_spec, context);
        }
    }
}

impl ServiceTargetPort {
    fn check_pod_spec_contains_numeric_port(
        service: &Service,
        pod_spec: &PodSpec,
        context: &Context,
    ) {
        for ports in f!(service.spec, ports).unwrap_or(&vec![]) {
            if let Some(IntOrString::Int(port_number)) = ports.target_port {
                Self::report_if_exists_on_spec(port_number, service, pod_spec, context)
            }
        }
    }

    fn report_if_exists_on_spec(
        port_number: i32,
        service: &Service,
        pod_spec: &PodSpec,
        context: &Context,
    ) {
        for container in pod_spec.containers.iter() {
            for port in container.ports.as_ref().unwrap_or(&vec![]) {
                if port_number != port.container_port {
                    continue;
                }

                let finding = Finding::new(LINT_NAME, service.metadata.clone());
                context.reporter.report(finding);
            }
        }
    }

    fn has_numeric_port(service: &Service) -> bool {
        f!(service.spec, ports)
            .unwrap_or(&vec![])
            .iter()
            .any(|port| {
                port.target_port
                    .as_ref()
                    .map(|target| match target {
                        IntOrString::Int(_) => true,
                        _ => false,
                    })
                    .unwrap_or(false)
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{analyze_file, filter_findings_by};
    use std::path::Path;

    #[test]
    fn it_finds_services_without_named_port() {
        let findings = analyze_file(Path::new("../tests/service_target_port.yaml"));
        let findings = filter_findings_by(findings, super::LINT_NAME);

        assert_eq!(1, findings.len());
        assert_eq!("service-not-using-named-port", findings[0].name());
    }
}
