mod config;
mod error;
mod kube;
mod linters;
mod reporting;
#[cfg(test)]
mod tests;
mod view;
mod visitor;

use crate::config::Config;
use crate::error::KorrecteError;
use crate::kube::api::{ApiObjectRepository, FrozenObjectRepository};
use crate::kube::file::FileObjectRepository;
use crate::kube::ObjectRepository;
use crate::linters::evaluator::OneShotEvaluator;
use crate::linters::LintCollection;
use crate::reporting::Reporter;
use crate::view::cli::Cli;
use crate::view::View;
use ::kube::config as kube_config;
use clap::load_yaml;
use clap::{App, ArgMatches};
use std::borrow::Borrow;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml;

fn main() -> Result<(), KorrecteError> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let cfg_path = matches.value_of("config").unwrap_or("korrecte.toml");
    let cfg: Config = load_config(cfg_path).unwrap_or_else(|_| {
        println!("Could not load config file");
        Config::default()
    });

    let reporter = reporting::SingleThreadedReporter::default();
    let object_repository = build_object_repository(&matches)?;

    let list = LintCollection::all(cfg, &*object_repository);
    OneShotEvaluator::evaluate(&reporter, list, object_repository.borrow());

    let cli = Cli {};
    cli.render(&reporter.findings());
    Ok(())
}

fn build_object_repository(
    matches: &ArgMatches,
) -> Result<Box<dyn ObjectRepository>, KorrecteError> {
    match matches.value_of("source") {
        Some("api") | None => {
            let config = kube_config::load_kube_config()
                .map_err(|_| KorrecteError::Generic("Could not load kube config".into()))?;
            Ok(Box::new(FrozenObjectRepository::from(
                ApiObjectRepository::new(config)?,
            )))
        }
        Some("file") => {
            let path = matches
                .value_of("path")
                .ok_or_else(|| KorrecteError::Generic("Missing file path".into()))?;
            Ok(Box::new(FileObjectRepository::new(Path::new(path))?))
        }
        _ => Err(KorrecteError::Generic(
            "Could not build an object repository".into(),
        )),
    }
}

fn load_config(path: &str) -> Result<Config, KorrecteError> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(toml::from_str(&buffer)?)
}
