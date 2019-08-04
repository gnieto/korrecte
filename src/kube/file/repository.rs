use std::path::Path;
use crate::kube::{ObjectRepository, Identifier, KubeObjectType};
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use k8s_openapi::api::core::v1::{ServiceSpec, ServiceStatus};
use kube::api::Object;
use crate::kube::file::KubeObjectLoader;
use crate::error::KorrecteError;

#[derive(Clone)]
pub struct FileObjectRepository {
    objects: Vec<KubeObjectType>,
}

impl ObjectRepository for FileObjectRepository {
    fn pod(&self, _id: &Identifier) -> Option<Object<PodSpec, PodStatus>> {
        unimplemented!()
    }

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

    fn service(&self, _id: &Identifier) -> Option<Object<ServiceSpec, ServiceStatus>> {
        unimplemented!()
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
}

impl FileObjectRepository {
    pub fn new(path: &Path) -> Result<FileObjectRepository, KorrecteError> {
        let objects = KubeObjectLoader::read_file(&path)?;

        let properly_parsed_objects: Vec<KubeObjectType> = objects
            .iter()
            .filter_map(|object| object.as_ref().ok().cloned())
            .collect();

        Ok(FileObjectRepository {
            objects: properly_parsed_objects,
        })
    }
}