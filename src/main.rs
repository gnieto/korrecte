mod linters;
mod config;
mod reporting;

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

fn main() {
    let mut file = File::open("korrecte.toml").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let cfg: Config = toml::from_str(&buffer).unwrap();

    let reporter = reporting::SingleThreadedReporter::default();

    let config = kube_config::load_kube_config().expect("failed to load kubeconfig");
    let client = APIClient::new(config);

    // Manage pods
    let pods = Api::v1Pod(client).within("default")
        .list(&ListParams::default())
        .unwrap();

    let required = linters::required_labels::RequiredLabels::new(cfg.required_labels.clone(), reporter.clone());

    for p in pods.items.iter() {
        required.pod(p);
    }


    for f in reporter.findings().iter() {
        println!("Found something on: {}", f.object_metadata().name);
    }

    println!("Hello, world!");
}
