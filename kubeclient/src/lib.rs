pub mod config;
mod k8s;
mod kubeclient;

use crate::config::KubeClientError;
use crate::kubeclient::Object;
pub use kubeclient::{KubeClient, KubernetesError};
use serde::de::DeserializeOwned;

pub struct Reflector<O: Object> {
    client: KubeClient,
    objects: Option<Vec<O>>,
    resource_version: Option<String>,
}

impl<O: Object + Clone + DeserializeOwned> Reflector<O> {
    pub fn new(client: KubeClient) -> Self {
        Reflector {
            client,
            objects: None,
            resource_version: None,
        }
    }

    pub async fn get(&mut self) -> Result<Vec<O>, KubeClientError> {
        if self.objects.is_none() {
            self.initial_load().await?;
        }

        Ok(self.objects.clone().unwrap())
    }

    async fn initial_load(&mut self) -> Result<(), KubeClientError> {
        let list_resource = self
            .client
            .list::<O>()
            .await
            .map_err(|_| KubeClientError::Serialization)?;

        self.objects = Some(list_resource.items);
        self.resource_version = list_resource.metadata.resource_version;

        Ok(())
    }
}
