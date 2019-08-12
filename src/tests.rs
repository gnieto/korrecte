use crate::linters::LintCollection;
use crate::reporting::{Finding, SingleThreadedReporter, Reporter};
use crate::kube::file::FileObjectRepository;
use std::path::Path;
use crate::linters::evaluator::OneShotEvaluator;
use crate::kube::ObjectRepository;

pub fn analyze_file(path: &Path) -> Vec<Finding> {
    let reporter = SingleThreadedReporter::default();
    let repository: Box<dyn ObjectRepository> = Box::new(FileObjectRepository::new(path).unwrap());
    let ll = LintCollection::all(crate::config::Config::default(), &repository);

    OneShotEvaluator::evaluate(&reporter, ll, &repository);

    reporter.findings()
}