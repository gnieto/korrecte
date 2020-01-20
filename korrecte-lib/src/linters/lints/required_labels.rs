use crate::linters::Lint;

use crate::f;
use crate::linters::evaluator::Context;
use crate::reporting::Finding;
use k8s_openapi::api::core::v1::Pod;
use serde::Deserialize;
use std::collections::HashMap;

pub(crate) struct RequiredLabels {
    config: Config,
}

impl RequiredLabels {
    pub fn new(config: Config) -> Self {
        RequiredLabels { config }
    }
}

impl Lint for RequiredLabels {
    fn name(&self) -> &str {
        "required_labels"
    }

    fn core_v1_pod(&self, pod: &Pod, context: &Context) {
        let current_labels: Vec<String> = f!(pod.metadata, labels)
            .map(|labels| labels.keys().cloned().collect())
            .unwrap_or_default();

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

            let finding = Finding::new(self.name(), pod.metadata.clone()).with_metadata(metadata);

            context.reporter.report(finding);
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
