use crate::linters::Lint;

use crate::f;
use crate::linters::evaluator::Context;
use crate::reporting::Finding;
use crate::visitor::{visit_all_pod_specs, PodSpecVisitor};
use k8s_openapi::api::core::v1::PodSpec;
use k8s_openapi::api::core::v1::Service;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
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

        let mut visitor = MatchingPodSpec::new(&selectors);
        visit_all_pod_specs(context, &mut visitor);

        if !visitor.any_pod_matches {
            let finding = Finding::new(self.name(), service.metadata.clone());
            context.reporter.report(finding);
        }
    }
}

struct MatchingPodSpec<'a> {
    selector: &'a BTreeMap<String, String>,
    any_pod_matches: bool,
}

impl<'a> MatchingPodSpec<'a> {
    pub fn new(selector: &'a BTreeMap<String, String>) -> Self {
        MatchingPodSpec {
            selector,
            any_pod_matches: false,
        }
    }
}

impl<'a> PodSpecVisitor for MatchingPodSpec<'a> {
    fn visit_pod_spec(&mut self, _: &PodSpec, pod_meta: &ObjectMeta, _: Option<&ObjectMeta>) {
        if self.any_pod_matches {
            // If we found any podspec which matches with the given labels, we do not need
            // to do any further check
            return;
        }

        if let Some(pod_labels) = pod_meta.labels.as_ref() {
            self.any_pod_matches = self.selector.iter().all(|(k, v)| {
                pod_labels
                    .get(k)
                    .map(|pod_value| pod_value == v)
                    .unwrap_or(false)
            });
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
