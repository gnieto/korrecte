use crate::linters::{Lint, LintSpec, Group};

use kube::api::Object;
use k8s_openapi::api::core::v1::{ServiceSpec, ServiceStatus};
use crate::reporting::{Reporter, Finding};
use std::collections::BTreeMap;
use crate::kube::ObjectRepository;

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
    object_repository: &'a Box<dyn ObjectRepository>,
}

impl<'a> ServiceWithoutMatchingLabels<'a> {
    pub fn new(object_repository: &'a Box<dyn ObjectRepository>) -> Self {
        ServiceWithoutMatchingLabels {
            object_repository,
        }
    }
}

impl<'a> Lint for ServiceWithoutMatchingLabels<'a> {
    fn spec(&self) -> LintSpec {
        LintSpec {
            group: Group::Configuration,
            name: "service_without_matching_labels".to_string(),
        }
    }

    fn service(&self, service: &Object<ServiceSpec, ServiceStatus>, reporter: &dyn Reporter) {
        let selectors: BTreeMap<String, String> = service.spec.selector.clone().unwrap_or_default();

        let any_matching_pod = self.object_repository.pods()
            .iter()
            .any(|pod| {
                let pod_labels = &pod.metadata.labels;

                selectors.iter()
                    .all(|(k, v)| {
                        pod_labels.get(k)
                            .map(|pod_value| pod_value == v)
                            .unwrap_or(false)
                    })
            });

        if !any_matching_pod {
            let finding= Finding::new(self.spec().clone(), service.metadata.clone());
            reporter.report(finding);
        }
    }
}