use crate::linters::{KubeObjectType, Lint};

use crate::linters::evaluator::Context;
use crate::reporting::Finding;
use crate::visitor::{pod_spec_visit, PodSpecVisitor};
use k8s_openapi::api::core::v1::PodSpec;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

#[derive(Default)]
pub(crate) struct NeverRestartWithLivenessProbe;

const LINT_NAME: &str = "never_restart_with_liveness_probe";

impl Lint for NeverRestartWithLivenessProbe {
    fn name(&self) -> &str {
        LINT_NAME
    }

    fn object(&self, object: &KubeObjectType, context: &Context) {
        let mut visitor = NeverRestartWithLivenessProbeVisitor { context };
        pod_spec_visit(&object, &mut visitor);
    }
}

struct NeverRestartWithLivenessProbeVisitor<'a> {
    context: &'a Context<'a>,
}

impl<'a> PodSpecVisitor for NeverRestartWithLivenessProbeVisitor<'a> {
    fn visit_pod_spec(&mut self, pod_spec: &PodSpec, _: &ObjectMeta, meta: Option<&ObjectMeta>) {
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

        let finding = Finding::new(LINT_NAME, meta.cloned());
        self.context.reporter.report(finding);
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{analyze_file, filter_findings_by};
    use std::path::Path;

    #[test]
    fn it_finds_never_restart_errors() {
        let findings = analyze_file(Path::new("../tests/never_restart.yaml"));
        let findings = filter_findings_by(findings, super::LINT_NAME);

        assert_eq!(1, findings.len());
        assert_eq!(findings[0].name(), "hello-node-never-restart");
    }
}
