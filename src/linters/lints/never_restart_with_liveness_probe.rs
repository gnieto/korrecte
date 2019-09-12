use crate::linters::{Group, Lint, LintSpec};

use crate::reporting::Finding;
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use kube::api::Object;

/// **What it does:** Finds pods which have a `Never` restart policy and have liveness probe set
///
/// **Why is this bad?** Those containers which have a liveness probe will be stopped if the probe
///  fails and it will never be restarted, which may lead the pod on a inconsistent state.
///
/// **Known problems:**
///
/// **References:**
/// - https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#container-probes
#[derive(Default)]
pub(crate) struct NeverRestartWithLivenessProbe;

impl Lint for NeverRestartWithLivenessProbe {
    fn v1_pod(&self, pod: &Object<PodSpec, PodStatus>) -> Vec<Finding> {
        let mut findings = Vec::new();
        let restart_policy: String = pod
            .spec
            .restart_policy
            .clone()
            .unwrap_or_else(|| "Always".to_string());
        if restart_policy.to_ascii_lowercase() != "never" {
            return findings;
        }

        let has_any_liveness_probe = pod
            .spec
            .containers
            .iter()
            .any(|c| c.liveness_probe.is_some());

        if !has_any_liveness_probe {
            return findings;
        }

        let finding = Finding::new(self.spec().clone(), pod.metadata.clone());
        findings.push(finding);

        findings
    }

    fn spec(&self) -> LintSpec {
        LintSpec {
            group: Group::Configuration,
            name: "never_restart_with_liveness_probe".to_string(),
        }
    }
}
