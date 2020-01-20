use anyhow::*;
use colored::*;
use korrecte::linters::LintSpecLoader;
use korrecte::reporting::Finding;

pub struct Cli;

impl Cli {
    pub fn render(findings: &[Finding]) -> Result<()> {
        let lint_specs = LintSpecLoader::new()?;

        for finding in findings {
            let spec = lint_specs
                .get(finding.lint_name())
                .ok_or_else(|| anyhow!("Missing spec for finding"))?;

            println!(
                "{} on {} [{}]. Metadata: {:?}",
                spec.name.bold(),
                finding.name().green(),
                finding
                    .namespace()
                    .as_ref()
                    .unwrap_or(&"default".to_string())
                    .blue(),
                finding.lint_metadata(),
            )
        }

        Ok(())
    }
}
