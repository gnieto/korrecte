mod linters;
mod config;

use kube::{
    api::{Api, Informer, WatchEvent, Object},
    client::APIClient,
    config as kube_config,
};
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use kube::api::ListParams;
use serde_json;
use crate::linters::Lint;
use toml;
use std::fs::File;
use std::io::prelude::*;
use crate::config::Config;

fn main() {
    let mut file = File::open("korrecte.toml").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer);

    let cfg: Config = toml::from_str(&buffer).unwrap();
    dbg!(&cfg);

    let config = kube_config::load_kube_config().expect("failed to load kubeconfig");
    let client = APIClient::new(config);

    // Manage pods
    let pods = Api::v1Pod(client).within("default")
        .list(&ListParams::default())
        .unwrap();

    let required = linters::required_labels::RequiredLabels::new(cfg.required_labels.clone());

    for p in pods.items.iter() {
        required.pod(p);
        println!("{:?}", serde_json::to_string(&p.metadata).unwrap());
    }

    println!("Hello, world!");
}
