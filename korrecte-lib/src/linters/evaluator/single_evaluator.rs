use crate::kube::ObjectRepository;
use crate::linters::evaluator::{Context, Evaluator};
use crate::linters::LintList;
use crate::reporting::Reporter;

pub struct SingleEvaluator;

impl Evaluator for SingleEvaluator {
    fn evaluate(
        &self,
        reporter: &dyn Reporter,
        list: &LintList,
        object_repository: &dyn ObjectRepository,
    ) {
        let context = Context {
            reporter,
            repository: object_repository,
        };

        for lint in list.iter() {
            for object in object_repository.iter() {
                lint.object(&object, &context);
            }
        }
    }
}
