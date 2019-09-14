use crate::kube::file::FileObjectRepository;
use crate::kube::ObjectRepository;
use crate::linters::evaluator::OneShotEvaluator;
use crate::linters::{LintCollection, LintSpec};
use crate::reporting::{Finding, Reporter, SingleThreadedReporter};
use std::borrow::Borrow;
use std::path::Path;

pub fn analyze_file(path: &Path) -> Vec<Finding> {
    let reporter = SingleThreadedReporter::default();
    let repository: Box<dyn ObjectRepository> = Box::new(FileObjectRepository::new(path).unwrap());
    let ll = LintCollection::all(crate::config::Config::default(), repository.borrow());

    OneShotEvaluator::evaluate(&reporter, ll, repository.borrow());

    reporter.findings()
}

pub fn filter_findings_by(findings: Vec<Finding>, spec: &LintSpec) -> Vec<Finding> {
    findings.into_iter().filter(|f| f.spec() == spec).collect()
}
