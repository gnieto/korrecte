use crate::linters::KubeObjectType;
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

pub(crate) struct KubeObjectLoader;
pub(crate) type LoadResult = Result<Vec<Result<KubeObjectType>>>;

impl KubeObjectLoader {
    pub fn read(input: &str) -> LoadResult {
        let mut output = Vec::new();
        let decoded_yaml = YamlLoader::load_from_str(input)?;

        for yaml_object in decoded_yaml {
            let kube_object_type = Self::yaml_object_to_kube_object_type(&yaml_object);
            if kube_object_type.is_err() {
                dbg!(&yaml_object);
            }
            output.push(kube_object_type);
        }

        Ok(output)
    }

    pub fn read_file(path: &Path) -> LoadResult {
        // TODO: It could be interesting to be able to read a dir instead
        let file_content = fs::read_to_string(path)?;

        Self::read(file_content.as_str())
    }

    fn yaml_object_to_kube_object_type(yaml: &Yaml) -> Result<KubeObjectType> {
        let mut out_str = String::new();
        let (api_version, kind) = {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(&yaml)?;

            let hash = yaml
                .as_hash()
                .ok_or_else(|| anyhow!("Exepected a hash object on the root"))?;
            let api_version = hash
                .get(&Yaml::String("apiVersion".to_string()))
                .and_then(|api_version| api_version.as_str())
                .ok_or_else(|| anyhow!("Could not find apiVersion field"))?;
            let kind = hash
                .get(&Yaml::String("kind".to_string()))
                .and_then(|kind| kind.as_str())
                .ok_or_else(|| anyhow!("Could not find kind field"))?;

            (api_version, kind)
        };

        KubeObjectType::from_yaml(&out_str, api_version, kind)
    }
}
