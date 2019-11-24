use crate::kube::file::KubeObjectLoader;
use crate::kube::ObjectRepository;
use crate::linters::KubeObjectType;
use anyhow::Result;
use std::path::Path;

pub struct FileObjectRepository {
    objects: Vec<KubeObjectType>,
}

impl FileObjectRepository {
    pub fn new(path: &Path) -> Result<FileObjectRepository> {
        let objects = if path.is_dir() {
            let objects: Vec<Result<KubeObjectType>> = path
                .read_dir()?
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
            .into_iter()
            .filter_map(|object| {
                if object.is_err() {
                    println!(
                        "Could not decode some of the objects on file: {} {:?}",
                        path.display(),
                        object.err().unwrap()
                    );
                    return None;
                }

                object.ok()
            })
            .collect();

        Ok(FileObjectRepository {
            objects: properly_parsed_objects,
        })
    }
}

impl ObjectRepository for FileObjectRepository {
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a KubeObjectType> + 'a> {
        Box::new(self.objects.iter())
    }
}
