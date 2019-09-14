use crate::linters::{Group, KubeObjectType, Lint, LintSpec};

use crate::kube::ObjectRepository;
use crate::reporting::Finding;
use k8s_openapi::api::core::v1::{ServiceSpec, ServiceStatus};
use kube::api::Object;
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
pub(crate) struct ServiceWithoutMatchingLabels<'a> {
    object_repository: &'a dyn ObjectRepository,
}

impl<'a> ServiceWithoutMatchingLabels<'a> {
    pub fn new(object_repository: &'a dyn ObjectRepository) -> Self {
        ServiceWithoutMatchingLabels { object_repository }
    }
}

impl<'a> Lint for ServiceWithoutMatchingLabels<'a> {
    fn v1_service(&self, service: &Object<ServiceSpec, ServiceStatus>) -> Vec<Finding> {
        let mut findings = Vec::new();
        let selectors: BTreeMap<String, String> = service.spec.selector.clone().unwrap_or_default();

        let any_matching_pod = self
            .object_repository
            .all()
            .iter()
            .filter_map(|object| match object {
                KubeObjectType::V1Pod(p) => Some(p),
                _ => None,
            })
            .any(|pod| {
                let pod_labels = &pod.metadata.labels;

                selectors.iter().all(|(k, v)| {
                    pod_labels
                        .get(k)
                        .map(|pod_value| pod_value == v)
                        .unwrap_or(false)
                })
            });

        if !any_matching_pod {
            let finding = Finding::new(Self::spec(), service.metadata.clone());
            findings.push(finding);
        }

        findings
    }
}

impl<'a> ServiceWithoutMatchingLabels<'a> {
    fn spec() -> LintSpec {
        LintSpec {
            group: Group::Configuration,
            name: "service_without_matching_labels".to_string(),
        }
    }
}
