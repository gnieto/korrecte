use crate::linters::{Group, KubeObjectType, Lint, LintSpec};

use crate::linters::evaluator::Context;
use crate::reporting::Finding;
use crate::visitor::{pod_spec_visit, PodSpecVisitor};
use k8s_openapi::api::core::v1::PodSpec;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

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
    fn object(&self, object: &KubeObjectType, context: &Context) {
        let mut visitor = NeverRestartWithLivenessProbeVisitor { context };
        pod_spec_visit(&object, &mut visitor);
    }
}

struct NeverRestartWithLivenessProbeVisitor<'a> {
    context: &'a Context<'a>,
}

impl<'a> PodSpecVisitor for NeverRestartWithLivenessProbeVisitor<'a> {
    fn visit_pod_spec(&mut self, pod_spec: &PodSpec, meta: Option<&ObjectMeta>) {
        let restart_policy: String = pod_spec
            .restart_policy
            .clone()
            .unwrap_or_else(|| "Always".to_string());
        if restart_policy.to_ascii_lowercase() != "never" {
            return;
        }

        let has_any_liveness_probe = pod_spec
            .containers
            .iter()
            .any(|c| c.liveness_probe.is_some());

        if !has_any_liveness_probe {
            return;
        }

        let finding = Finding::new(NeverRestartWithLivenessProbe::spec(), meta.cloned());
        self.context.reporter.report(finding);
    }
}

impl NeverRestartWithLivenessProbe {
    fn spec() -> LintSpec {
        LintSpec {
            group: Group::Configuration,
            name: "never_restart_with_liveness_probe".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::linters::lints::never_restart_with_liveness_probe::NeverRestartWithLivenessProbe;
    use crate::tests::{analyze_file, filter_findings_by};
    use std::path::Path;

    #[test]
    fn it_finds_never_restart_errors() {
        let findings = analyze_file(Path::new("../tests/never_restart.yaml"));
        let findings = filter_findings_by(findings, &NeverRestartWithLivenessProbe::spec());

        assert_eq!(1, findings.len());
        assert_eq!(findings[0].name(), "hello-node-never-restart");
    }
}
