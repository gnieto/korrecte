use crate::config::Config;
use crate::kube::file::FileObjectRepository;
use crate::kube::ObjectRepository;
use crate::linters::evaluator::{Context, Evaluator, SingleEvaluator};
use crate::linters::LintCollection;
use crate::reporting::{Finding, Reporter, SingleThreadedReporter};
use std::borrow::Borrow;
use std::path::Path;

pub fn analyze_file_cfg(path: &Path, config: Config) -> Vec<Finding> {
    let reporter = SingleThreadedReporter::default();
    let repository: Box<dyn ObjectRepository> = Box::new(FileObjectRepository::new(path).unwrap());
    let context = Context {
        repository: repository.borrow(),
        reporter: &reporter,
        config: &config.korrecte,
    };

    let ll = LintCollection::all(&config);

    let evaluator = SingleEvaluator;
    evaluator.evaluate(&context, &ll);

    reporter.findings()
}

pub fn analyze_file(path: &Path) -> Vec<Finding> {
    analyze_file_cfg(path, Config::default())
}

pub fn filter_findings_by(findings: Vec<Finding>, name: &str) -> Vec<Finding> {
    findings
        .into_iter()
        .filter(|f| f.lint_name() == name)
        .collect()
}
