use crate::kube::{KubeVersion, ObjectRepository};
use crate::linters::KubeObjectType;
use ::pin_utils::pin_mut;
use anyhow::*;
use futures::future::Future;
use kubeclient::config::load_config;
use kubeclient::KubeClient;
use std::borrow::Borrow;
use std::pin::Pin;
use std::str::FromStr;

pub struct ApiObjectRepository {
    kubeclient: KubeClient,
}

impl ApiObjectRepository {
    pub fn new() -> Result<Self> {
        let config = load_config().with_context(|| "Could not load kubernetes config")?;
        let kubeclient = KubeClient::new(config.borrow()).with_context(|| {
            "Could not create a kubeclient with the given configuration".to_string()
        })?;

        Ok(Self { kubeclient })
    }

    pub async fn load_all_objects(&self) -> Result<Vec<KubeObjectType>, ()> {
        let mut v: Vec<
            Pin<&mut dyn Future<Output = Result<Vec<KubeObjectType>, (String, anyhow::Error)>>>,
        > = Vec::new();
        let mut objects = Vec::new();

        let node = async {
            let pods = self
                .kubeclient
                .list::<k8s_openapi::api::core::v1::Node>()
                .await;

            pods.map(|list| {
                list.items
                    .into_iter()
                    .map(|p| KubeObjectType::CoreV1Node(Box::new(p)))
                    .collect::<Vec<KubeObjectType>>()
            })
            .map_err(|e| ("node".to_string(), e))
        };
        pin_mut!(node);
        v.push(node);

        let pod = async {
            let pods = self
                .kubeclient
                .list::<k8s_openapi::api::core::v1::Pod>()
                .await;

            pods.map(|list| {
                list.items
                    .into_iter()
                    .map(|p| KubeObjectType::CoreV1Pod(Box::new(p)))
                    .collect::<Vec<KubeObjectType>>()
            })
            .map_err(|e| ("pod".to_string(), e))
        };
        pin_mut!(pod);
        v.push(pod);

        let service = async {
            let pods = self
                .kubeclient
                .list::<k8s_openapi::api::core::v1::Service>()
                .await;

            pods.map(|list| {
                list.items
                    .into_iter()
                    .map(|p| KubeObjectType::CoreV1Service(Box::new(p)))
                    .collect::<Vec<KubeObjectType>>()
            })
            .map_err(|e| ("service".to_string(), e))
        };
        pin_mut!(service);
        v.push(service);

        let daemon_set = async {
            let pods = self
                .kubeclient
                .list::<k8s_openapi::api::apps::v1::DaemonSet>()
                .await;

            pods.map(|list| {
                list.items
                    .into_iter()
                    .map(|p| KubeObjectType::AppsV1DaemonSet(Box::new(p)))
                    .collect::<Vec<KubeObjectType>>()
            })
            .map_err(|e| ("daemon_set".to_string(), e))
        };
        pin_mut!(daemon_set);
        v.push(daemon_set);

        let deployment = async {
            let pods = self
                .kubeclient
                .list::<k8s_openapi::api::apps::v1::Deployment>()
                .await;

            pods.map(|list| {
                list.items
                    .into_iter()
                    .map(|p| KubeObjectType::AppsV1Deployment(Box::new(p)))
                    .collect::<Vec<KubeObjectType>>()
            })
            .map_err(|e| ("deployment".to_string(), e))
        };
        pin_mut!(deployment);
        v.push(deployment);

        let replica_set = async {
            let pods = self
                .kubeclient
                .list::<k8s_openapi::api::apps::v1::ReplicaSet>()
                .await;

            pods.map(|list| {
                list.items
                    .into_iter()
                    .map(|p| KubeObjectType::AppsV1ReplicaSet(Box::new(p)))
                    .collect::<Vec<KubeObjectType>>()
            })
            .map_err(|e| ("replica_set".to_string(), e))
        };
        pin_mut!(replica_set);
        v.push(replica_set);

        let stateful_set = async {
            let pods = self
                .kubeclient
                .list::<k8s_openapi::api::apps::v1::StatefulSet>()
                .await;

            pods.map(|list| {
                list.items
                    .into_iter()
                    .map(|p| KubeObjectType::AppsV1StatefulSet(Box::new(p)))
                    .collect::<Vec<KubeObjectType>>()
            })
            .map_err(|e| ("stateful_set".to_string(), e))
        };
        pin_mut!(stateful_set);
        v.push(stateful_set);

        let pod_disruption_budget = async {
            let pods = self
                .kubeclient
                .list::<k8s_openapi::api::policy::v1beta1::PodDisruptionBudget>()
                .await;

            pods.map(|list| {
                list.items
                    .into_iter()
                    .map(|p| KubeObjectType::PolicyV1beta1PodDisruptionBudget(Box::new(p)))
                    .collect::<Vec<KubeObjectType>>()
            })
            .map_err(|e| ("pod_disruption_budget".to_string(), e))
        };
        pin_mut!(pod_disruption_budget);
        v.push(pod_disruption_budget);

        let horizontal_pod_autoscaler = async {
            let pods = self
                .kubeclient
                .list::<k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler>()
                .await;

            pods.map(|list| {
                list.items
                    .into_iter()
                    .map(|p| KubeObjectType::AutoscalingV1HorizontalPodAutoscaler(Box::new(p)))
                    .collect::<Vec<KubeObjectType>>()
            })
            .map_err(|e| ("horizontal_pod_autoscaler".to_string(), e))
        };
        pin_mut!(horizontal_pod_autoscaler);
        v.push(horizontal_pod_autoscaler);

        let ingress = async {
            let pods = self
                .kubeclient
                .list::<k8s_openapi::api::networking::v1beta1::Ingress>()
                .await;

            pods.map(|list| {
                list.items
                    .into_iter()
                    .map(|p| KubeObjectType::NetworkingV1beta1Ingress(Box::new(p)))
                    .collect::<Vec<KubeObjectType>>()
            })
            .map_err(|e| ("ingress".to_string(), e))
        };
        pin_mut!(ingress);
        v.push(ingress);

        let ingress = async {
            let pods = self
                .kubeclient
                .list::<k8s_openapi::api::extensions::v1beta1::Ingress>()
                .await;

            pods.map(|list| {
                list.items
                    .into_iter()
                    .map(|p| KubeObjectType::ExtensionsV1beta1Ingress(Box::new(p)))
                    .collect::<Vec<KubeObjectType>>()
            })
            .map_err(|e| ("ingress".to_string(), e))
        };
        pin_mut!(ingress);
        v.push(ingress);

        let cluster_role = async {
            let pods = self
                .kubeclient
                .list::<k8s_openapi::api::rbac::v1::ClusterRole>()
                .await;

            pods.map(|list| {
                list.items
                    .into_iter()
                    .map(|p| KubeObjectType::RbacV1ClusterRole(Box::new(p)))
                    .collect::<Vec<KubeObjectType>>()
            })
            .map_err(|e| ("cluster_role".to_string(), e))
        };
        pin_mut!(cluster_role);
        v.push(cluster_role);

        let role = async {
            let pods = self
                .kubeclient
                .list::<k8s_openapi::api::rbac::v1::Role>()
                .await;

            pods.map(|list| {
                list.items
                    .into_iter()
                    .map(|p| KubeObjectType::RbacV1Role(Box::new(p)))
                    .collect::<Vec<KubeObjectType>>()
            })
            .map_err(|e| ("role".to_string(), e))
        };
        pin_mut!(role);
        v.push(role);

        let a: Vec<Result<Vec<KubeObjectType>, (String, anyhow::Error)>> =
            futures::future::join_all(v).await;

        for r in a {
            if r.is_err() {
                let (ty, _) = r.err().unwrap();
                println!("Found some error while loading {} from kubernetes", ty);
                continue;
            }

            let res = r.unwrap();
            objects.extend(res);
        }

        Ok(objects)
    }
}

pub struct FrozenObjectRepository {
    objects: Vec<KubeObjectType>,
    version: Option<KubeVersion>,
}

impl From<ApiObjectRepository> for FrozenObjectRepository {
    fn from(api: ApiObjectRepository) -> Self {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let version = rt.block_on(api.kubeclient.version()).unwrap();
        let major = u16::from_str(&version.major);
        let minor = u16::from_str(&version.minor);
        let version = match (major, minor) {
            (Ok(maj), Ok(min)) => Some(KubeVersion::new(maj, min)),
            _ => None,
        };

        let all_objects = rt.block_on(api.load_all_objects()).unwrap();

        FrozenObjectRepository {
            objects: all_objects,
            version,
        }
    }
}

impl ObjectRepository for FrozenObjectRepository {
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a KubeObjectType> + 'a> {
        Box::new(self.objects.iter())
    }

    fn version(&self) -> Option<&KubeVersion> {
        self.version.as_ref()
    }
}
