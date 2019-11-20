use crate::linters::{Group, KubeObjectType, Lint, LintSpec};

use crate::f;
use crate::linters::evaluator::Context;
use crate::reporting::Finding;
use k8s_openapi::api::core::v1::Service;
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

        let any_matching_pod = context
            .repository
            .iter()
            .filter_map(|object| match object {
                KubeObjectType::V1Pod(p) => Some(p),
                _ => None,
            })
            .any(|pod| {
                // let pod_labels = &pod.metadata.unwrap_or_default().labels.unwrap_or_default();
                let pod_labels = f!(pod.metadata, labels).cloned().unwrap_or_default();

                selectors.iter().all(|(k, v)| {
                    pod_labels
                        .get(k)
                        .map(|pod_value| pod_value == v)
                        .unwrap_or(false)
                })
            });

        if !any_matching_pod {
            let finding = Finding::new(Self::spec(), service.metadata.clone());
            context.reporter.report(finding);
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
