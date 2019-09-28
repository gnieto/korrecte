use crate::linters::{Group, Lint, LintSpec};

use crate::reporting::Finding;
use crate::reporting::Reporter;
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use kube::api::Object;
use serde::Deserialize;
use std::collections::HashMap;

/// **What it does:** Checks for missing required labels
///
/// **Why is this bad?** Adding labels to your pods helps organizing the cluster and
/// improves long-term maintainability.
///
/// **Known problems:** None
///
/// **References**
/// https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#motivation
pub(crate) struct RequiredLabels {
    config: Config,
}

impl RequiredLabels {
    pub fn new(config: Config) -> Self {
        RequiredLabels { config }
    }

    fn spec() -> LintSpec {
        LintSpec {
            group: Group::Audit,
            name: "required_labels".to_string(),
        }
    }
}

impl Lint for RequiredLabels {
    fn v1_pod(&self, pod: &Object<PodSpec, PodStatus>, reporter: &dyn Reporter) {
        let current_labels: Vec<String> = pod.metadata.labels.keys().cloned().collect();

        let missing_labels: Vec<String> = self
            .config
            .labels
            .iter()
            .filter(|label| !current_labels.contains(label))
            .cloned()
            .collect();

        if !missing_labels.is_empty() {
            let mut metadata = HashMap::new();
            metadata.insert(
                "missing_labels".to_string(),
                format!("{:?}", missing_labels),
            );

            let finding =
                Finding::from_object_metadata(RequiredLabels::spec(), pod.metadata.clone()).with_metadata(metadata);

            reporter.report(finding);
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Config {
    #[serde(default = "default_labels")]
    labels: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            labels: default_labels(),
        }
    }
}

fn default_labels() -> Vec<String> {
    vec!["app".to_string(), "role".to_string()]
}
