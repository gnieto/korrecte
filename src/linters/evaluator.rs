use crate::linters::LintList;
use crate::kube::ObjectRepository;
use crate::reporting::{Reporter, Finding};

pub struct OneShotEvaluator;

impl OneShotEvaluator {
    pub fn evaluate(reporter: &dyn Reporter, list: LintList, object_repository: &Box<dyn ObjectRepository>) {
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