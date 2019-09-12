use crate::kube::ObjectRepository;
use crate::linters::KubeObjectType;
use k8s_openapi::api::apps;
use k8s_openapi::api::autoscaling;
use k8s_openapi::api::core;
use kube::api::{Api, KubeObject, Object, Reflector};
use kube::client::APIClient;
use kube::config::Configuration;
use kube::Result;
use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct ApiObjectRepository {
    node: Reflector<Object<core::v1::NodeSpec, core::v1::NodeStatus>>,
    pod: Reflector<Object<core::v1::PodSpec, core::v1::PodStatus>>,
    service: Reflector<Object<core::v1::ServiceSpec, core::v1::ServiceStatus>>,
    deployment: Reflector<Object<apps::v1::DeploymentSpec, apps::v1::DeploymentStatus>>,
    horizontal_pod_autoscaler: Reflector<
        Object<
            autoscaling::v1::HorizontalPodAutoscalerSpec,
            autoscaling::v1::HorizontalPodAutoscalerStatus,
        >,
    >,
}

impl ApiObjectRepository {
    pub fn new(kube_config: Configuration) -> Result<Self> {
        let client = APIClient::new(kube_config);

        Ok(ApiObjectRepository {
            node: ApiObjectRepository::initialize_reflector(Api::v1Node(client.clone()))?,
            pod: ApiObjectRepository::initialize_reflector(Api::v1Pod(client.clone()))?,
            service: ApiObjectRepository::initialize_reflector(Api::v1Service(client.clone()))?,
            deployment: ApiObjectRepository::initialize_reflector(Api::v1Deployment(
                client.clone(),
            ))?,
            horizontal_pod_autoscaler: ApiObjectRepository::initialize_reflector(
                Api::v1HorizontalPodAutoscaler(client.clone()),
            )?,
        })
    }

    fn initialize_reflector<K: 'static + Send + Sync + Clone + DeserializeOwned + KubeObject>(
        api: Api<K>,
    ) -> Result<Reflector<K>> {
        let mut pod_reflect = Reflector::new(api);
        pod_reflect = pod_reflect.init()?;
        let pod_reflect_updater = pod_reflect.clone();

        std::thread::spawn(move || loop {
            if let Err(e) = pod_reflect_updater.poll() {
                println!("Error while updating pods: {}", e.to_string());
            }
        });

        Ok(pod_reflect)
    }
}

pub struct FrozenObjectRepository {
    objects: Vec<KubeObjectType>,
}

impl From<ApiObjectRepository> for FrozenObjectRepository {
    fn from(api: ApiObjectRepository) -> Self {
        let mut objects = Vec::new();

        objects.extend(
            api.node
                .read()
                .unwrap()
                .iter()
                .map(|o| KubeObjectType::V1Node(Box::new(o.clone()))),
        );
        objects.extend(
            api.pod
                .read()
                .unwrap()
                .iter()
                .map(|o| KubeObjectType::V1Pod(Box::new(o.clone()))),
        );
        objects.extend(
            api.service
                .read()
                .unwrap()
                .iter()
                .map(|o| KubeObjectType::V1Service(Box::new(o.clone()))),
        );
        objects.extend(
            api.deployment
                .read()
                .unwrap()
                .iter()
                .map(|o| KubeObjectType::V1Deployment(Box::new(o.clone()))),
        );
        objects.extend(
            api.horizontal_pod_autoscaler
                .read()
                .unwrap()
                .iter()
                .map(|o| KubeObjectType::V1HorizontalPodAutoscaler(Box::new(o.clone()))),
        );

        FrozenObjectRepository { objects }
    }
}

impl ObjectRepository for FrozenObjectRepository {
    fn all(&self) -> &Vec<KubeObjectType> {
        &self.objects
    }
}
