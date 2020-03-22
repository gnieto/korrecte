use crate::linters::Lint;

use crate::f;
use crate::kube::service::FindFistMatchingPodSpec;
use crate::linters::evaluator::Context;
use crate::reporting::Finding;
use crate::visitor::visit_all_pod_specs;
use k8s_openapi::api::core::v1::Service;
use std::collections::BTreeMap;

const LINT_NAME: &str = "service_without_matching_labels";

pub(crate) struct ServiceWithoutMatchingLabels;

impl Lint for ServiceWithoutMatchingLabels {
    fn name(&self) -> &str {
        LINT_NAME
    }

    fn core_v1_service(&self, service: &Service, context: &Context) {
        let selectors: BTreeMap<String, String> =
            f!(service.spec, selector).cloned().unwrap_or_default();

        let mut visitor = FindFistMatchingPodSpec::new(&selectors);
        visit_all_pod_specs(context, &mut visitor);

        if visitor.first_matching_pod_spec().is_none() {
            let finding = Finding::new(self.name(), service.metadata.clone());
            context.reporter.report(finding);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{analyze_file, filter_findings_by};
    use std::path::Path;

    #[test]
    fn it_finds_services_without_matching_labels() {
        let findings = analyze_file(Path::new("../tests/service_without_matching_labels.yml"));
        let findings = filter_findings_by(findings, super::LINT_NAME);

        assert_eq!(2, findings.len());
        assert_eq!("my-service", findings[0].name());
        assert_eq!("multi-tag-non-match", findings[1].name());
    }
}
