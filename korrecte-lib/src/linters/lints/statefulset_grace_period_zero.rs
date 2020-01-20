use crate::linters::Lint;

use crate::linters::evaluator::Context;
use crate::reporting::Finding;
use crate::{f, m};
use k8s_openapi::api::apps::v1::StatefulSet;

const LINT_NAME: &str = "statefulset_no_grace_period";

#[derive(Default)]
pub(crate) struct StatefulsetGracePeriodZero;

impl Lint for StatefulsetGracePeriodZero {
    fn name(&self) -> &str {
        LINT_NAME
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

#[cfg(test)]
mod test {
    use crate::tests::{analyze_file, filter_findings_by};
    use std::path::Path;

    #[test]
    pub fn it_detects_statefulset_with_graceperiod_zero() {
        let findings = analyze_file(Path::new("../tests/statefulset_graceperiod.yaml"));
        let findings = filter_findings_by(findings, super::LINT_NAME);

        assert_eq!(1, findings.len());
        assert_eq!(findings[0].lint_name(), super::LINT_NAME);
        assert_eq!(findings[0].name(), "web");
    }
}
