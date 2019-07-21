mod linters;
mod config;
mod reporting;
mod view;

use kube::{
    api::Api,
    client::APIClient,
    config as kube_config,
};
use kube::api::ListParams;
use crate::linters::Lint;
use toml;
use std::fs::File;
use std::io::prelude::*;
use crate::config::Config;
use crate::reporting::Reporter;
use crate::view::cli::Cli;
use crate::view::View;

fn main() {
    let cfg: Config = load_config().unwrap_or_default();

    let reporter = reporting::SingleThreadedReporter::default();

    let config = kube_config::load_kube_config().expect("failed to load kubeconfig");
    let client = APIClient::new(config);

    // Manage pods
    let pods = Api::v1Pod(client).within("default")
        .list(&ListParams::default())
        .unwrap();

    let required = linters::lints::required_labels::RequiredLabels::new(cfg.required_labels.clone(), reporter.clone());
    let overlapping = linters::lints::overlapping_probes::OverlappingProbes::new(reporter.clone());
    let never = linters::lints::never_restart_with_liveness_probe::NeverRestartWithLivenessProbe::new(reporter.clone());

    for p in pods.items.iter() {
        required.pod(p);
        overlapping.pod(p);
        never.pod(p);
    }

    let cli = Cli {};
    cli.render(&reporter.findings());
}

fn load_config() -> Result<Config, ConfigError> {
    let mut file = File::open("korrecte.toml")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(toml::from_str(&buffer)?)
}

enum ConfigError {
    Io(std::io::Error),
    Serde(toml::de::Error),
}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        ConfigError::Serde(e)
    }
}