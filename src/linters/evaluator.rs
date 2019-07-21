use kube::{
    api::Api,
    client::APIClient,
    config,
};
use kube::api::ListParams;
use crate::linters::LintList;

pub struct OneShotEvaluator;

impl OneShotEvaluator {
    pub fn evaluate(list: LintList) {
        let config = config::load_kube_config().expect("failed to load kubeconfig");
        let client = APIClient::new(config);

        // Manage pods
        let pods = Api::v1Pod(client)
            .list(&ListParams::default())
            .unwrap();

        for p in pods.items.iter() {
            for lint in list.iter() {
                lint.pod(p);
            }
        }
    }
}