use crate::linters::{Group, Lint, LintSpec};

use crate::reporting::Finding;
use crate::reporting::Reporter;
use k8s_openapi::api::apps::v1::{StatefulSetSpec, StatefulSetStatus};
use kube::api::{KubeObject, Object};

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
    fn v1_stateful_set(
        &self,
        stateful_set: &Object<StatefulSetSpec, StatefulSetStatus>,
        reporter: &dyn Reporter,
    ) {
        if let Some(ref spec) = stateful_set.spec.template.spec {
            let grace_period = spec.termination_grace_period_seconds.unwrap_or(1);

            if grace_period == 0 {
                let finding = Finding::new(Self::spec(), stateful_set.meta().clone());
                reporter.report(finding);
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
        assert_eq!(findings[0].object_metadata().name, "web");
    }
}
