
use kube::api::{Object, Reflector, KubeObject, Api};
use kube::config::Configuration;
use kube::client::APIClient;
use kube::Result;
use k8s_openapi::api::core;
use k8s_openapi::api::apps;
use k8s_openapi::api::autoscaling;
use serde::de::DeserializeOwned;
use super::Identifier;
use crate::linters::KubeObjectType;
use crate::kube::ObjectRepository;

#[derive(Clone)]
pub struct ApiObjectRepository {
	node: Reflector<Object<core::v1::NodeSpec, core::v1::NodeStatus>>,
	pod: Reflector<Object<core::v1::PodSpec, core::v1::PodStatus>>,
	service: Reflector<Object<core::v1::ServiceSpec, core::v1::ServiceStatus>>,
	deployment: Reflector<Object<apps::v1::DeploymentSpec, apps::v1::DeploymentStatus>>,
	horizontal_pod_autoscaler: Reflector<Object<autoscaling::v1::HorizontalPodAutoscalerSpec, autoscaling::v1::HorizontalPodAutoscalerStatus>>,
}

impl ApiObjectRepository {
    pub fn new(kube_config: Configuration) -> Result<Self> {
        let client = APIClient::new(kube_config);

        Ok(ApiObjectRepository {
			node: ApiObjectRepository::initialize_reflector(Api::v1Node(client.clone()))?,
			pod: ApiObjectRepository::initialize_reflector(Api::v1Pod(client.clone()))?,
			service: ApiObjectRepository::initialize_reflector(Api::v1Service(client.clone()))?,
			deployment: ApiObjectRepository::initialize_reflector(Api::v1Deployment(client.clone()))?,
			horizontal_pod_autoscaler: ApiObjectRepository::initialize_reflector(Api::v1HorizontalPodAutoscaler(client.clone()))?,
        })
    }

    fn initialize_reflector<K: 'static + Send + Sync + Clone + DeserializeOwned + KubeObject>(api: Api<K>) -> Result<Reflector<K>> {
        let mut pod_reflect = Reflector::new(api);
        pod_reflect = pod_reflect.init()?;
        let pod_reflect_updater = pod_reflect.clone();

        std::thread::spawn(move || {
            loop {
                if let Err(e) = pod_reflect_updater.poll() {
                    println!("Error while updating pods: {}", e.to_string());
                }
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
            api.node.read()
                    .unwrap()
                    .iter()
                    .map(|o| {
                        KubeObjectType::V1Node(o.clone())
                    })
        );
		objects.extend(
            api.pod.read()
                    .unwrap()
                    .iter()
                    .map(|o| {
                        KubeObjectType::V1Pod(o.clone())
                    })
        );
		objects.extend(
            api.service.read()
                    .unwrap()
                    .iter()
                    .map(|o| {
                        KubeObjectType::V1Service(o.clone())
                    })
        );
		objects.extend(
            api.deployment.read()
                    .unwrap()
                    .iter()
                    .map(|o| {
                        KubeObjectType::V1Deployment(o.clone())
                    })
        );
		objects.extend(
            api.horizontal_pod_autoscaler.read()
                    .unwrap()
                    .iter()
                    .map(|o| {
                        KubeObjectType::V1HorizontalPodAutoscaler(o.clone())
                    })
        );

        FrozenObjectRepository {
            objects,
        }
    }
}

impl ObjectRepository for FrozenObjectRepository {
    fn all(&self) -> &Vec<KubeObjectType> {
        &self.objects
    }

    fn find(&self, _id: &Identifier) -> Option<&KubeObjectType> {
        unimplemented!()
    }
}
