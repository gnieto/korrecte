use crate::config::reqwest::reqwest_client;
use crate::config::ClusterConfig;
use anyhow::{Context, Result};
use reqwest::Client as ReqwestClient;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::borrow::Borrow;
use k8s_openapi::Resource;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta;

pub struct KubeClient {
    base_uri: String,
    client: ReqwestClient,
}

impl KubeClient {
    pub fn new(config: &dyn ClusterConfig) -> Result<KubeClient> {
        let client = reqwest_client(config.borrow())?;
        let kube_client = Self {
            base_uri: config.base_uri().to_string(),
            client,
        };

        Ok(kube_client)
    }
}

impl KubeClient {
    pub async fn list<O: Object + DeserializeOwned>(&self) -> Result<ObjectList<O>> {
        let path = O::path(None);

        let response = self.client.get(&self.build_url(&path)).send().await?;

        let body = response.text().await?;

        serde_json::from_str(&body).context("Error deserializing response")
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.base_uri, path)
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
            resource = Self::name(),
        )
    }

    fn prefix() -> &'static str;

    fn name() -> &'static str;

    fn is_namespaced() -> bool;
}
