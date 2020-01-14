use crate::linters::{Group, Lint, LintSpec};

use crate::linters::evaluator::Context;
use crate::reporting::Finding;
use crate::{f, m};
use k8s_openapi::api::apps::v1::StatefulSet;

/// **What it does:** Finds stateful sets which has a pod template with graceful period equals to zero
///
/// **Why is this bad?** Stateful Sets are usually used on clustered applications in which each of the components
/// have state. This kind of application needs a proper shutdown with a given timeout, otherwise, the application
/// may lead to an inconsistent state.
///
/// **Known problems:**
///
/// **References**
/// https://kubernetes.io/docs/tasks/run-application/force-delete-stateful-set-pod/#delete-pods
#[derive(Default)]
pub(crate) struct StatefulsetGracePeriodZero;

impl Lint for StatefulsetGracePeriodZero {
    fn name(&self) -> &str {
        "statefulset_no_grace_period"
    }

    fn apps_v1_stateful_set(&self, stateful_set: &StatefulSet, context: &Context) {
        if let Some(ref spec) = m!(stateful_set.spec, template, spec) {
            let grace_period = f!(spec, termination_grace_period_seconds)
                .cloned()
                .unwrap_or(1);

            if grace_period == 0 {
                let finding = Finding::new(self.name(), stateful_set.metadata.clone());
                context.reporter.report(finding);
            }
        }
    }
}

impl StatefulsetGracePeriodZero {
    fn spec() -> LintSpec {
        LintSpec {
            group: Group::Configuration,
            name: "statefulset_no_grace_period".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::linters::lints::statefulset_grace_period_zero::StatefulsetGracePeriodZero;
    use crate::tests::{analyze_file, filter_findings_by};
    use std::path::Path;

    #[test]
    pub fn it_detects_statefulset_with_graceperiod_zero() {
        let findings = analyze_file(Path::new("../tests/statefulset_graceperiod.yaml"));
        let findings = filter_findings_by(findings, &StatefulsetGracePeriodZero::spec());

        assert_eq!(1, findings.len());
        assert_eq!(findings[0].spec().name, "statefulset_no_grace_period");
        assert_eq!(findings[0].name(), "web");
    }
}
