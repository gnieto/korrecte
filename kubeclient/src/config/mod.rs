use crate::config::file::inline_or_file;
use anyhow::{anyhow, Context as AnyhowContext, Result};
use openssl::pkcs12::Pkcs12;
use openssl::pkey::PKey;
use openssl::x509::X509;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use thiserror::Error;

pub mod file;
pub mod reqwest;

pub fn load_config() -> Result<Config> {
    load_local_config().or_else(|_| load_incluster())
}

pub fn load_incluster() -> Result<Config> {
    Err(anyhow!("In cluster config loading not yet implemented"))
}

pub fn load_local_config() -> Result<Config> {
    let path = get_local_config_path()?;
    let file = File::open(&path).with_context(|| format!("File {:?} could not be opened", path))?;

    serde_yaml::from_reader(file)
        .with_context(|| format!("File {:?} does not contain a valid yaml file", path))
}

fn get_local_config_path() -> Result<PathBuf> {
    dirs::home_dir()
        .map(|h| h.join(".kube").join("config"))
        .ok_or_else(|| anyhow!("Could not find .kube/config file"))
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    kind: String,
    #[serde(rename = "apiVersion")]
    api_version: String,
    preferences: Preferences,
    clusters: Vec<NamedCluster>,
    #[serde(rename = "users")]
    auth_info: Vec<NamedAuthInfo>,
    contexts: Vec<NamedContext>,
    current_context: Option<String>,
}

#[derive(Debug, Error)]
pub enum ResolutionError {
    #[error("Missing current context")]
    MissingCurrentContext,
    #[error("Context {0} not found")]
    MissingContext(String),
    #[error("Cluster {0} not found")]
    MissingCluster(String),
    #[error("Auth {0} not found")]
    MissingAuth(String),
}

impl Config {
    pub fn resolve(&self) -> Result<CurrentConfig, ResolutionError> {
        let current_context = self
            .current_context
            .as_ref()
            .ok_or_else(|| ResolutionError::MissingCurrentContext)?;
        let context = self
            .find_context(&current_context)
            .ok_or_else(|| ResolutionError::MissingContext(current_context.to_string()))?;
        let cluster = self
            .find_cluster(&context.cluster)
            .ok_or_else(|| ResolutionError::MissingCluster(context.cluster.to_string()))?;
        let auth = self
            .find_auth(&context.auth_info)
            .ok_or_else(|| ResolutionError::MissingAuth(context.auth_info.to_string()))?;

        let current_config = CurrentConfig {
            cluster: cluster.clone(),
            context: context.clone(),
            auth: auth.clone(),
        };
        Ok(current_config)
    }

    fn find_context(&self, current_context: &str) -> Option<&Context> {
        self.contexts
            .iter()
            .find(|named_context| named_context.name == current_context)
            .map(|named_context| &named_context.context)
    }

    fn find_cluster(&self, cluster_name: &str) -> Option<&Cluster> {
        self.clusters
            .iter()
            .find(|named_cluster| named_cluster.name == cluster_name)
            .map(|named_cluster| &named_cluster.cluster)
    }

    fn find_auth(&self, auth_name: &str) -> Option<&AuthInfo> {
        self.auth_info
            .iter()
            .find(|named_auth| named_auth.name == auth_name)
            .map(|named_auth| &named_auth.auth_info)
    }
}

#[derive(Debug)]
pub struct CurrentConfig {
    pub cluster: Cluster,
    pub context: Context,
    pub auth: AuthInfo,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Preferences {
    #[serde(default)]
    pub colors: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Cluster {
    server: String,
    #[serde(default)]
    insecure_skip_tls_verify: bool,
    certificate_authority: Option<String>,
    certificate_authority_data: Option<Vec<u8>>,
}

impl Cluster {
    pub fn certificate_authority(&self) -> Result<Vec<u8>> {
        inline_or_file(
            &self.certificate_authority_data,
            &self.certificate_authority,
        )
        .context("Could not load certificate authority data")
    }

    pub fn insecure_skip_tls_verify(&self) -> bool {
        self.insecure_skip_tls_verify
    }

    pub fn server(&self) -> &String {
        &self.server
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AuthInfo {
    client_certificate: Option<String>,
    client_certificate_data: Option<Vec<u8>>,
    client_key: Option<String>,
    client_key_data: Option<Vec<u8>>,
    token: Option<String>,
    token_file: Option<String>,
    impersonate: Option<String>,
    #[serde(default)]
    impersonate_groups: Vec<String>,
    #[serde(default)]
    impersonate_user_extra: HashMap<String, Vec<String>>,
    username: Option<String>,
    password: Option<String>,
    auth_provider: Option<AuthProviderConfig>,
    exec: Option<ExecConfig>,
}

impl AuthInfo {
    pub fn identity(&self) -> Result<Pkcs12> {
        let certificate = inline_or_file(&self.client_certificate_data, &self.client_certificate)?;
        let client_key = inline_or_file(&self.client_key_data, &self.client_key)?;

        let x509 = X509::from_pem(&certificate).context("Certificate is not a x509 pem file")?;
        let client_key = PKey::private_key_from_pem(&client_key)
            .context("Identity private key could not be read")?;

        Pkcs12::builder()
            .build("", "identity", &client_key, &x509)
            .context("Error building pkcs12")
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct NamedCluster {
    name: String,
    cluster: Cluster,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct NamedAuthInfo {
    name: String,
    #[serde(rename = "user")]
    auth_info: AuthInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Context {
    pub cluster: String,
    #[serde(rename = "user")]
    auth_info: String,
    namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct NamedContext {
    name: String,
    context: Context,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AuthProviderConfig {
    name: String,
    config: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ExecConfig {
    command: String,
    args: Vec<String>,
    env: Vec<ExecEnvVar>,
    #[serde(rename = "apiVersion")]
    api_version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ExecEnvVar {
    name: String,
    value: String,
}
