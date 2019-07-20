mod linters;

use kube::{
    api::{Api, Informer, WatchEvent, Object},
    client::APIClient,
    config,
};
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use kube::api::ListParams;
use serde_json;
use crate::linters::Lint;

fn main() {
    let config = config::load_kube_config().expect("failed to load kubeconfig");
    let client = APIClient::new(config);

    // Manage pods
    let pods = Api::v1Pod(client).within("default")
        .list(&ListParams::default())
        .unwrap();

    let required = linters::required_labels::RequiredLabels::new(vec!["some".to_string(), "label".to_string()]);

    for p in pods.items.iter() {
        required.pod(p);
        println!("{:?}", serde_json::to_string(&p.metadata).unwrap());
    }

    println!("Hello, world!");
}
