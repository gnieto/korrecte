use crate::config::reqwest::reqwest_client;
use crate::config::CurrentConfig;
use crate::config::KubeClientError;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta;
use k8s_openapi::Resource;
use reqwest::Client as ReqwestClient;
use serde::de::DeserializeOwned;
use serde::Deserialize;

pub struct KubeClient {
    config: CurrentConfig,
    client: ReqwestClient,
}

impl KubeClient {
    pub fn new(config: CurrentConfig) -> Result<KubeClient, KubeClientError> {
        let client = reqwest_client(&config)?;

        let kube_client = Self { config, client };

        Ok(kube_client)
    }
}

#[derive(Debug)]
pub struct KubernetesError;

impl From<reqwest::Error> for KubernetesError {
    fn from(_: reqwest::Error) -> Self {
        KubernetesError
    }
}

impl From<serde_json::error::Error> for KubernetesError {
    fn from(_: serde_json::error::Error) -> Self {
        KubernetesError
    }
}

impl KubeClient {
    pub async fn list<O: Object + DeserializeOwned>(
        &self,
    ) -> Result<ObjectList<O>, KubernetesError> {
        let path = O::path(None);

        let response = self.client.get(&self.build_url(&path)).send().await?;

        let body = response.text().await?;

        serde_json::from_str(&body).map_err(|_| KubernetesError)
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.config.cluster.server(), path)
    }
}

#[derive(Deserialize)]
pub struct ObjectList<O: Object> {
    #[serde(bound(deserialize = "Vec<O>: Deserialize<'de>"))]
    pub items: Vec<O>,

    pub metadata: ListMeta,
}

pub trait Object: Resource + Sized {
    fn path(namespace: Option<&str>) -> String {
        let pref = if Self::prefix() == "" {
            "".into()
        } else {
            format!("{}/", Self::prefix())
        };
        let g = if Self::group() == "" {
            "".into()
        } else {
            format!("{}/", Self::group())
        };
        let n = if let Some(ns) = namespace {
            format!("namespaces/{}/", ns)
        } else {
            "".into()
        };

        format!(
            "/{prefix}{group}{version}/{namespaces}{resource}",
            prefix = pref,
            group = g,
            version = Self::version(),
            namespaces = n,
            resource = Self::resource(),
        )
    }

    fn prefix() -> &'static str;

    fn resource() -> String {
        format!("{}s", Self::kind().to_lowercase())
    }
}
