use crate::linters::{Lint, LintSpec, Group};

use kube::api::Object;
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use crate::reporting::{Reporter, Finding};

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
    fn spec(&self) -> LintSpec {
        LintSpec {
            group: Group::Configuration,
            name: "never_restart_with_liveness_probe".to_string(),
        }
    }

    fn pod(&self, pod: &Object<PodSpec, PodStatus>, reporter: &dyn Reporter) {
        let restart_policy: String = pod.spec.restart_policy.clone().unwrap_or("Always".to_string());
        if restart_policy.to_ascii_lowercase() != "never" {
            return
        }

        let has_any_liveness_probe = pod.spec.containers
            .iter()
            .any(|c| c.liveness_probe.is_some());

        if !has_any_liveness_probe {
            return
        }

        let finding = Finding::new(self.spec().clone(), pod.metadata.clone());
        reporter.report(finding);
    }
}