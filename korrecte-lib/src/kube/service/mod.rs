use std::collections::BTreeMap;
use crate::visitor::PodSpecVisitor;
use k8s_openapi::api::core::v1::PodSpec;
use kube::api::ObjectMeta;

pub(crate) struct FindFistMatchingPodSpec<'a> {
    selector: &'a BTreeMap<String, String>,
    any_pod_matches: Option<PodSpec>,
}

impl<'a> FindFistMatchingPodSpec<'a> {
    pub fn new(selector: &'a BTreeMap<String, String>) -> Self {
        FindFistMatchingPodSpec {
            selector,
            any_pod_matches: None,
        }
    }

    pub fn first_matching_pod_spec(&self) -> Option<&PodSpec> {
        self.any_pod_matches.as_ref()
    }
}

impl<'a> PodSpecVisitor for FindFistMatchingPodSpec<'a> {
    fn visit_pod_spec(&mut self, pod: &PodSpec, pod_meta: &ObjectMeta, _: Option<&ObjectMeta>) {
        if self.any_pod_matches.is_some() {
            // If we found any podspec which matches with the given labels, we do not need
            // to do any further check
            return;
        }

        if let Some(pod_labels) = pod_meta.labels.as_ref() {
            let has_matching_labels = self.selector.iter().all(|(k, v)| {
                pod_labels
                    .get(k)
                    .map(|pod_value| pod_value == v)
                    .unwrap_or(false)
            });

            if has_matching_labels {
               self.any_pod_matches = Some(pod.clone());
            }
        }
    }
}