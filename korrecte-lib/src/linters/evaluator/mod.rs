use crate::kube::ObjectRepository;
use crate::linters::LintList;
use crate::reporting::Reporter;

mod single_evaluator;

pub use crate::linters::evaluator::single_evaluator::SingleEvaluator;

#[allow(clippy::ptr_arg)]
pub trait Evaluator {
    fn evaluate(
        &self,
        reporter: &dyn Reporter,
        list: &LintList,
        object_repository: &dyn ObjectRepository,
    );
}

pub struct Context<'a> {
    pub repository: &'a dyn ObjectRepository,
    pub reporter: &'a dyn Reporter,
}
