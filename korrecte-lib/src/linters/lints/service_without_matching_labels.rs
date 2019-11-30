use crate::linters::{Group, Lint, LintSpec};

use crate::f;
use crate::linters::evaluator::Context;
use crate::reporting::Finding;
use crate::visitor::{visit_all_pod_specs, PodSpecVisitor};
use k8s_openapi::api::core::v1::PodSpec;
use k8s_openapi::api::core::v1::Service;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use std::collections::BTreeMap;

/// **What it does:** Checks that services are well defined and has some matching
/// object (defined by the service selector).
///
/// **Why is this bad?** A service without any matching pod is usually a symptom of a
/// bad configuration
///
/// **Known problems:** Sending data to that service may provoke failures
///
/// **References**
pub(crate) struct ServiceWithoutMatchingLabels;

impl Lint for ServiceWithoutMatchingLabels {
    fn v1_service(&self, service: &Service, context: &Context) {
        let selectors: BTreeMap<String, String> =
            f!(service.spec, selector).cloned().unwrap_or_default();

        let mut visitor = MatchingPodSpec::new(&selectors);
        visit_all_pod_specs(context, &mut visitor);

        if !visitor.any_pod_matches {
            let finding = Finding::new(Self::spec(), service.metadata.clone());
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

impl ServiceWithoutMatchingLabels {
    fn spec() -> LintSpec {
        LintSpec {
            group: Group::Configuration,
            name: "service_without_matching_labels".to_string(),
        }
    }
}
