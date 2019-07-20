use super::{Lint, LintSpec, Group};

use kube::api::Object;
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use serde::Deserialize;

impl RequiredLabels {
    pub fn new(config: Config) -> Self {
        RequiredLabels {
            config,
        }
    }
}

impl Lint for RequiredLabels {
    fn spec(&self) -> LintSpec {
        LintSpec {
            group: Group::Audit,
        }
    }

    fn pod(&self, pod: &Object<PodSpec, PodStatus>) {
        let current_labels: Vec<String> = pod.metadata
            .labels
            .keys()
            .cloned()
            .collect();

        let missing_labels: Vec<String> = self.config.labels.iter()
            .filter(|label| !current_labels.contains(label))
            .cloned()
            .collect();

        if !missing_labels.is_empty() {
            println!("Missing labels: {:?}", missing_labels);
            // Report lint matching
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

pub(crate) struct RequiredLabels {
    config: Config,
}