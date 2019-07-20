use super::Lint;

use kube::api::Object;
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};

pub(crate) struct RequiredLabels {
    required_labels: Vec<String>,
}

impl RequiredLabels {
    pub fn new(required_labels: Vec<String>) -> Self {
        RequiredLabels {
            required_labels,
        }
    }
}

impl Lint for RequiredLabels {
    fn pod(&self, pod: &Object<PodSpec, PodStatus>) {
        let current_labels: Vec<String> = pod.metadata
            .labels
            .keys()
            .cloned()
            .collect();

        let missing_labels: Vec<String> = current_labels.iter()
            .filter(|label| !self.required_labels.contains(label))
            .cloned()
            .collect();

        if !missing_labels.is_empty() {
            // Report lint matching
        }
    }
}