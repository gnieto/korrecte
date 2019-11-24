
use kube::api::{Object, Reflector, KubeObject, Api};
use kube::client::APIClient;
use kube::Result;
use k8s_openapi::api::apps;
use k8s_openapi::api::core;
use k8s_openapi::api::autoscaling;
use serde::de::DeserializeOwned;
use crate::linters::KubeObjectType;
use crate::kube::ObjectRepository;
use ::kube::config as kube_config;

#[derive(Clone)]
pub struct ApiObjectRepository {
	node: Reflector<Object<core::v1::NodeSpec, core::v1::NodeStatus>>,
	pod: Reflector<Object<core::v1::PodSpec, core::v1::PodStatus>>,
	service: Reflector<Object<core::v1::ServiceSpec, core::v1::ServiceStatus>>,
	daemon_set: Reflector<Object<apps::v1::DaemonSetSpec, apps::v1::DaemonSetStatus>>,
	deployment: Reflector<Object<apps::v1::DeploymentSpec, apps::v1::DeploymentStatus>>,
	replica_set: Reflector<Object<apps::v1::ReplicaSetSpec, apps::v1::ReplicaSetStatus>>,
	stateful_set: Reflector<Object<apps::v1::StatefulSetSpec, apps::v1::StatefulSetStatus>>,
	horizontal_pod_autoscaler: Reflector<Object<autoscaling::v1::HorizontalPodAutoscalerSpec, autoscaling::v1::HorizontalPodAutoscalerStatus>>,
}

impl ApiObjectRepository {
    pub fn new() -> Result<Self> {
        let kube_config = kube_config::load_kube_config()?;
        let client = APIClient::new(kube_config);

        Ok(ApiObjectRepository {
			node: ApiObjectRepository::initialize_reflector(Api::v1Node(client.clone()))?,
			pod: ApiObjectRepository::initialize_reflector(Api::v1Pod(client.clone()))?,
			service: ApiObjectRepository::initialize_reflector(Api::v1Service(client.clone()))?,
			daemon_set: ApiObjectRepository::initialize_reflector(Api::v1DaemonSet(client.clone()))?,
			deployment: ApiObjectRepository::initialize_reflector(Api::v1Deployment(client.clone()))?,
			replica_set: ApiObjectRepository::initialize_reflector(Api::v1ReplicaSet(client.clone()))?,
			stateful_set: ApiObjectRepository::initialize_reflector(Api::v1StatefulSet(client.clone()))?,
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

        FrozenObjectRepository {
            objects,
        }
    }
}

impl ObjectRepository for FrozenObjectRepository {
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=&'a KubeObjectType> + 'a> {
        Box::new(self.objects.iter())
    }
}