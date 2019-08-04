use crate::kube::KubeObjectType;
use crate::error::KorrecteError;
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};
use std::path::Path;
use std::fs;

pub(crate) struct KubeObjectLoader;
pub(crate) type LoadResult = Result<Vec<Result<KubeObjectType, KorrecteError>>, KorrecteError>;

impl KubeObjectLoader {
    pub fn read(input: &str) -> LoadResult {
        let mut output = Vec::new();
        let decoded_yaml = YamlLoader::load_from_str(input)
            .map_err(|_| KorrecteError::FailedToLoadYamlFile)?;

        for yaml_object in decoded_yaml {
            let kube_object_type = Self::yaml_object_to_kube_object_type(&yaml_object);
            output.push(kube_object_type);
        }

        Ok(output)
    }

    pub fn read_file(path: &Path) -> LoadResult {
        // TODO: It could be interesting to be able to read a dir instead
        let file_content = fs::read_to_string(path)
            .map_err(|_| KorrecteError::FailedToLoadYamlFile)?;

        Self::read(file_content.as_str())
    }

    fn yaml_object_to_kube_object_type(yaml: &Yaml) -> Result<KubeObjectType, KorrecteError> {
        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(&yaml).map_err(|_| KorrecteError::UnrecognizedObject)?;
        }

        serde_yaml::from_str(&out_str).map_err(|_| KorrecteError::UnrecognizedObject)
    }
}