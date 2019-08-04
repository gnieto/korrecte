mod linters;
mod config;
mod reporting;
mod view;
mod kube;
mod error;

use crate::linters::LintCollection;
use toml;
use std::fs::File;
use std::io::prelude::*;
use crate::config::Config;
use crate::reporting::Reporter;
use crate::view::cli::Cli;
use crate::view::View;
use crate::linters::evaluator::OneShotEvaluator;
use ::kube::config as kube_config;
use crate::kube::api::ApiObjectRepository;
use clap::{App, ArgMatches};
use clap::load_yaml;
use crate::kube::ObjectRepository;
use crate::kube::file::FileObjectRepository;
use std::path::Path;
use crate::error::KorrecteError;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let cfg_path = matches.value_of("config").unwrap_or("korrecte.toml");
    let cfg: Config = load_config(cfg_path).unwrap_or_else(|_| {
        println!("Could not load config file");
        Config::default()
    });
    let reporter = reporting::SingleThreadedReporter::default();

    let object_repository = build_object_repository(&matches).unwrap();
    let list = LintCollection::all(cfg, &object_repository);
    OneShotEvaluator::evaluate(&reporter, list, &object_repository);

    let cli = Cli {};
    cli.render(&reporter.findings());
}

fn build_object_repository(matches: &ArgMatches) -> Result<Box<dyn ObjectRepository>, KorrecteError> {
    match matches.value_of("source") {
        Some("api") => {
            let config = kube_config::load_kube_config().map_err( |_| KorrecteError::Generic("Could not load kube config".into()))?;
            Ok(Box::new(ApiObjectRepository::new(config)?))
        },
        Some("file") => {
            let path = matches.value_of("path").ok_or(KorrecteError::Generic("Missing file path".into()))?;
            Ok(Box::new(FileObjectRepository::new(Path::new(path))?))
        }
        _ => Err(KorrecteError::Generic("Could not build an object repository".into())),
    }
}

fn load_config(path: &str) -> Result<Config, KorrecteError> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(toml::from_str(&buffer)?)
}