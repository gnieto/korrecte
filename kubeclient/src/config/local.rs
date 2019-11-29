use crate::config::{AuthInfo, Cluster, ClusterConfig, Config, Context};
use anyhow::{anyhow, Context as ContextAnyhow, Result};
use openssl::pkcs12::Pkcs12;
use std::fs::File;
use std::path::PathBuf;

pub fn load_local_config() -> Result<RemoteCluster> {
    let path = get_local_config_path()?;
    let file = File::open(&path).with_context(|| format!("File {:?} could not be opened", path))?;

    let config: Config = serde_yaml::from_reader(file)
        .with_context(|| format!("File {:?} does not contain a valid yaml file", path))?;

    config
        .resolve()
        .with_context(|| "Could not find current cluster".to_string())
}

fn get_local_config_path() -> Result<PathBuf> {
    dirs::home_dir()
        .map(|h| h.join(".kube").join("config"))
        .ok_or_else(|| anyhow!("Could not find .kube/config file"))
}

#[derive(Debug)]
pub struct RemoteCluster {
    cluster: Cluster,
    context: Context,
    auth: AuthInfo,
}

impl RemoteCluster {
    pub fn new(cluster: Cluster, context: Context, auth: AuthInfo) -> Self {
        RemoteCluster {
            cluster,
            context,
            auth,
        }
    }
}

impl ClusterConfig for RemoteCluster {
    fn default_namespace(&self) -> Option<&String> {
        self.context.namespace.as_ref()
    }
    fn base_uri(&self) -> &String {
        self.cluster.server()
    }
    fn token(&self) -> Option<&String> {
        self.auth.token.as_ref()
    }
    fn certificate_authority(&self) -> Result<Vec<u8>> {
        self.cluster.certificate_authority()
    }
    fn skip_authority(&self) -> bool {
        self.cluster.insecure_skip_tls_verify()
    }
    fn identity(&self) -> Option<Pkcs12> {
        self.auth.identity().ok()
    }
}
