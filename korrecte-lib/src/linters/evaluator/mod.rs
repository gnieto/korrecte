use crate::kube::ObjectRepository;
use crate::linters::LintList;
use crate::reporting::Reporter;

mod single_evaluator;

use crate::config::KorrecteConfig;
pub use crate::linters::evaluator::single_evaluator::SingleEvaluator;

#[allow(clippy::ptr_arg)]
pub trait Evaluator {
    fn evaluate<'a>(&self, context: &'a Context<'a>, list: &LintList);
}

pub struct Context<'a> {
    pub repository: &'a dyn ObjectRepository,
    pub reporter: &'a dyn Reporter,
    pub config: &'a KorrecteConfig,
}
