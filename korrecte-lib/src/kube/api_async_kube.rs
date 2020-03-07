use crate::kube::ObjectRepository;
use crate::linters::KubeObjectType;
use anyhow::*;
use futures::future::Future;
use std::pin::Pin;
use kube::api::{Resource, ListParams, Meta};
use kube::runtime::Reflector;
use futures::FutureExt;
use serde::de::DeserializeOwned;

pub struct ApiObjectRepository {
    kubeclient: kube::client::APIClient,
}

type ReflectorFuture<'a> = Box<dyn Future<Output=Result<Vec<KubeObjectType>, anyhow::Error>> + 'a>;

impl ApiObjectRepository {
    pub fn new() -> Result<Self> {
        let config = futures::executor::block_on(kube::config::load_kube_config())?;
        let kubeclient = kube::client::APIClient::new(config);
        Ok(Self { kubeclient })
    }

    pub async fn load_all_objects(&self) -> Result<Vec<KubeObjectType>, ()> {
        let mut v: Vec<Pin<ReflectorFuture>> = Vec::new();
        let mut objects = Vec::new();

        v.push(self.reflector_for::<k8s_openapi::api::core::v1::Service>("svc").boxed());

        let all_futures: Vec<Result<Vec<KubeObjectType>, anyhow::Error>> =
            futures::future::join_all(v).await;

        for f in all_futures {
            match f {
                Err(ref e) => println!("Error loading some resource: {}", e),
                Ok(current) => objects.extend(current),
            }
        }

        Ok(objects)
    }

    pub async fn reflector_for<R: ReflectorFor>(&self, ty: &'static str) -> Result<Vec<KubeObjectType>, anyhow::Error> {
        let client = self.kubeclient.clone();

        let reflector = Reflector::<R>::new(
            client,
            ListParams::default(),
            Resource::all::<R>()
        );
        let reflector = reflector.init().await?;

        reflector
            .state()
            .await
            .map(|objects| {
                objects.iter()
                    .map(|obj| obj.clone().into())
                    .collect()
            })
            .map_err(|e| anyhow!("Err loading {}: {}", ty, e))
    }
}

pub trait ReflectorFor: Clone+Send+Meta+DeserializeOwned+Into<KubeObjectType> {}
impl<T: Clone+Send+Meta+DeserializeOwned+Into<KubeObjectType>> ReflectorFor for T {}

pub struct FrozenObjectRepository {
    objects: Vec<KubeObjectType>,
}

impl From<ApiObjectRepository> for FrozenObjectRepository {
    fn from(api: ApiObjectRepository) -> Self {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let all_objects = rt.block_on(api.load_all_objects()).unwrap();

        FrozenObjectRepository {
            objects: all_objects,
        }
    }
}

impl ObjectRepository for FrozenObjectRepository {
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a KubeObjectType> + 'a> {
        Box::new(self.objects.iter())
    }
}
