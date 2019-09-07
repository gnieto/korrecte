use super::View;
use crate::reporting::Finding;
use colored::*;

pub struct Cli;

impl View for Cli {
    fn render(&self, findings: &[Finding]) {
        for finding in findings {
            println!(
                "{} on {} [{}]. Metadata: {:?}",
                finding.spec().name.bold(),
                finding.object_metadata().name.as_ref().unwrap().green(),
                finding.object_metadata().namespace.as_ref().unwrap_or(&"default".to_string()).blue(),
                finding.lint_metadata(),
            )
        }
    }
}