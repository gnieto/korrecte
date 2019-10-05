mod error;
mod view;

use crate::error::CliError;
use clap::load_yaml;
use clap::{App, ArgMatches};
use korrecte::config::Config;
use korrecte::error::KorrecteError;
use korrecte::kube::api::{ApiObjectRepository, FrozenObjectRepository};
use korrecte::kube::file::FileObjectRepository;
use korrecte::kube::ObjectRepository;
use korrecte::linters::LintCollection;
use korrecte::reporting::Reporter;
use korrecte::reporting::SingleThreadedReporter;
use korrecte::view::View;
use std::borrow::Borrow;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml;

use crate::view::Cli;
use korrecte::linters::evaluator::{Evaluator, SingleEvaluator};

fn main() -> Result<(), CliError> {
    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let cfg_path = matches.value_of("config").unwrap_or("korrecte.toml");
    let cfg: Config = load_config(cfg_path).unwrap_or_else(|_| {
        println!("Could not load config file");
        Config::default()
    });

    let reporter = SingleThreadedReporter::default();
    let object_repository = build_object_repository(&matches)?;

    let list = LintCollection::all(cfg, &*object_repository);

    let evaluator = SingleEvaluator;
    evaluator.evaluate(&reporter, &list, object_repository.borrow());

    let cli = Cli {};
    cli.render(&reporter.findings());
    Ok(())
}

fn build_object_repository(
    matches: &ArgMatches,
) -> Result<Box<dyn ObjectRepository>, KorrecteError> {
    match matches.value_of("source") {
        Some("api") | None => Ok(Box::new(FrozenObjectRepository::from(
            ApiObjectRepository::new()?,
        ))),
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

fn load_config(path: &str) -> Result<Config, CliError> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(toml::from_str(&buffer)?)
}
