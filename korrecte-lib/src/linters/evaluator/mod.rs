use crate::kube::ObjectRepository;
use crate::linters::LintList;
use crate::reporting::Reporter;

mod single_evaluator;

pub use crate::linters::evaluator::single_evaluator::SingleEvaluator;

pub trait Evaluator {
    fn evaluate(
        &self,
        reporter: &dyn Reporter,
        list: &LintList,
        object_repository: &dyn ObjectRepository,
    );
}
