use crate::config::file::{load_file, load_from_file};
use crate::config::ClusterConfig;
use anyhow::Result;
use openssl::pkcs12::Pkcs12;

pub struct InClusterConfig {
    base_uri: String,
    namespace: Option<String>,
    token: String,
    certificate_authority: Vec<u8>,
}

impl InClusterConfig {
    pub fn new() -> Result<Self> {
        let host = std::env::var("KUBERNETES_SERVICE_HOST")?;
        let port = std::env::var("KUBERNETES_SERVICE_PORT")?;
        let base_uri = format!("https://{}:{}", host, port);

        Ok(InClusterConfig {
            base_uri,
            namespace: Self::load_namespace(),
            token: Self::load_token()?,
            certificate_authority: load_from_file(
                "/var/run/secrets/kubernetes.io/serviceaccount/ca.crt",
            )?,
        })
    }

    fn load_namespace() -> Option<String> {
        if let Ok(ns) = std::env::var("POD_NAMESPACE") {
            return Some(ns);
        }

        load_file("/var/run/secrets/kubernetes.io/serviceaccount/namespace")
            .ok()
            .map(|ns| ns.trim().to_string())
    }

    fn load_token() -> Result<String> {
        load_file("/var/run/secrets/kubernetes.io/serviceaccount/token")
    }
}

impl ClusterConfig for InClusterConfig {
    fn default_namespace(&self) -> Option<&String> {
        self.namespace.as_ref()
    }

    fn base_uri(&self) -> &String {
        &self.base_uri
    }

    fn token(&self) -> Option<&String> {
        Some(&self.token)
    }

    fn certificate_authority(&self) -> Result<Vec<u8>> {
        Ok(self.certificate_authority.clone())
    }

    fn skip_authority(&self) -> bool {
        false
    }

    fn identity(&self) -> Option<Pkcs12> {
        None
    }
}
