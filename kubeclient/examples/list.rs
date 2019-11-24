use k8s_openapi::api::core::v1::Pod;
use k8s_openapi::api::rbac::v1::ClusterRole;
use kubeclient::{KubeClient, Reflector};
use kubeclient_config::load_config;
use kubeclient_config::reqwest::reqwest_client;
use reqwest::{Client, ClientBuilder};

#[tokio::main]
async fn main() {
    let config = load_config().unwrap().resolve().unwrap();
    let kubeclient = KubeClient::new(config).unwrap();

    let mut pod_reflector = Reflector::<Pod>::new(kubeclient);
    let objects = pod_reflector.get().await.unwrap();

    for pod in objects {
        println!("Pod: {:?}", pod);
    }
}
