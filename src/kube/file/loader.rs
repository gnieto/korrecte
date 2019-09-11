use crate::error::KorrecteError;
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};
use std::path::Path;
use std::fs;
use crate::linters::KubeObjectType;

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
        let (api_version, kind) = {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(&yaml).map_err(|_| KorrecteError::UnrecognizedObject)?;

            let hash = yaml.as_hash().ok_or(KorrecteError::UnrecognizedObject)?;
            let api_version = hash.get(&Yaml::String("apiVersion".to_string()))
                .and_then(|api_version| api_version.as_str())
                .ok_or(KorrecteError::FailedToLoadYamlFile)?;
            let kind = hash.get(&Yaml::String("kind".to_string()))
                .and_then(|kind| kind.as_str())
                .ok_or(KorrecteError::FailedToLoadYamlFile)?;

            (api_version, kind)
        };

        KubeObjectType::from_yaml(&out_str, api_version, kind)
    }
}