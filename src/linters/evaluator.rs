use crate::kube::ObjectRepository;
use crate::linters::LintList;
use crate::reporting::{Finding, Reporter};

pub struct OneShotEvaluator;

impl OneShotEvaluator {
    pub fn evaluate(
        reporter: &dyn Reporter,
        list: LintList,
        object_repository: &dyn ObjectRepository,
    ) {
        for lint in list.iter() {
            for object in object_repository.all() {
                Self::report_lints(reporter, lint.object(&object));
            }
        }
    }

    fn report_lints(reporter: &dyn Reporter, findings: Vec<Finding>) {
        for f in findings {
            reporter.report(f);
        }
    }
}
