use std::path::Path;
use crate::kube::{ObjectRepository, KubeObjectType};
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use k8s_openapi::api::core::v1::{ServiceSpec, ServiceStatus};
use k8s_openapi::api::apps::v1::{DeploymentSpec, DeploymentStatus};
use k8s_openapi::api::autoscaling::v1::{HorizontalPodAutoscalerSpec, HorizontalPodAutoscalerStatus};
use k8s_openapi::api::policy::v1beta1::{PodDisruptionBudgetSpec, PodDisruptionBudgetStatus};
use kube::api::Object;
use crate::kube::file::KubeObjectLoader;
use crate::error::KorrecteError;

#[derive(Clone)]
pub struct FileObjectRepository {
    objects: Vec<KubeObjectType>,
}

impl ObjectRepository for FileObjectRepository {
    fn pods(&self) -> Vec<Object<PodSpec, PodStatus>> {
        self.objects
            .iter()
            .filter_map(|current_object| {
                match current_object {
                    KubeObjectType::Pod(pod) => {
                        Some(pod.clone())
                    },
                    _ => None,
                }
            })
            .collect()
    }

    fn services(&self) -> Vec<Object<ServiceSpec, ServiceStatus>> {
        self.objects
            .iter()
            .filter_map(|current_object| {
                match current_object {
                    KubeObjectType::Service(svc) => {
                        Some(svc.clone())
                    },
                    _ => None,
                }
            })
            .collect()
    }

    fn pod_disruption_budgets(&self) -> Vec<Object<PodDisruptionBudgetSpec, PodDisruptionBudgetStatus>> {
        self.objects
            .iter()
            .filter_map(|current_object| {
                match current_object {
                    KubeObjectType::PodDisruptionBudget(pdb) => {
                        Some(pdb.clone())
                    },
                    _ => None,
                }
            })
            .collect()
    }

    fn deployments(&self) -> Vec<Object<DeploymentSpec, DeploymentStatus>> {
        self.objects
            .iter()
            .filter_map(|current_object| {
                match current_object {
                    KubeObjectType::Deployment(deploy) => {
                        Some(deploy.clone())
                    },
                    _ => None,
                }
            })
            .collect()
    }

    fn horizontal_pod_autoscaler(&self) -> Vec<Object<HorizontalPodAutoscalerSpec, HorizontalPodAutoscalerStatus>> {
        self.objects
            .iter()
            .filter_map(|current_object| {
                match current_object {
                    KubeObjectType::HorizontalPodAutoscaler(hpa) => {
                        Some(hpa.clone())
                    },
                    _ => None,
                }
            })
            .collect()
    }
}

impl FileObjectRepository {
    pub fn new(path: &Path) -> Result<FileObjectRepository, KorrecteError> {
        let objects = if path.is_dir() {
            let objects: Vec<Result<KubeObjectType, KorrecteError>> = path.read_dir()?
                .into_iter()
                .map(|e| e.ok())
                .filter(|entry| entry.is_some())
                .map(|maybe_entry| maybe_entry.unwrap())
                .map(|entry| KubeObjectLoader::read_file(&entry.path()))
                .map(|objects| objects.unwrap_or_default())
                .flatten()
                .collect();

            objects
        } else if path.is_file() {
            KubeObjectLoader::read_file(&path)?
        } else {
            Vec::new()
        };

        let properly_parsed_objects: Vec<KubeObjectType> = objects
            .iter()
            .filter_map(|object| object.as_ref().ok().cloned())
            .collect();

        Ok(FileObjectRepository {
            objects: properly_parsed_objects,
        })
    }


}