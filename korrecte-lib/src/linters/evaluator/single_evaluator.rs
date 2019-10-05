use crate::kube::ObjectRepository;
use crate::linters::evaluator::Evaluator;
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
        for lint in list.iter() {
            for object in object_repository.all() {
                lint.object(&object, reporter);
            }
        }
    }
}
