mod view;

use anyhow::{anyhow, Result};
use clap::load_yaml;
use clap::{App, ArgMatches};
use korrecte::executor::{ExecutionContextBuilder, ExecutionMode, Executor};
use korrecte::reporting::Reporter;
use std::path::Path;

use crate::view::Cli;

fn main() -> Result<()> {
    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let ctx = ExecutionContextBuilder::default()
        .configuration_from_path(&Path::new(
            matches.value_of("config").unwrap_or("korrecte.toml"),
        ))?
        .execution_mode(
            get_execution_mode(&matches).ok_or_else(|| anyhow!("Invalid execution mode"))?,
        )
        .build();

    let executor = Executor::with_context(ctx);
    let reporter = executor.execute()?;

    Cli::render(&reporter.findings());

    Ok(())
}

fn get_execution_mode<'a>(matches: &'a ArgMatches) -> Option<ExecutionMode<'a>> {
    match matches.value_of("source") {
        Some("api") | None => Some(ExecutionMode::Api),
        Some("file") => {
            let path = matches.value_of("path")?;

            Some(ExecutionMode::FileSystem(Path::new(path)))
        }
        _ => None,
    }
}
