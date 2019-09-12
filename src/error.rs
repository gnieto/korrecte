#[derive(Debug)]
#[allow(unused)]
pub enum KorrecteError {
    Io(std::io::Error),
    Serde(toml::de::Error),
    KubeConfig(::kube::Error),
    Generic(String),
    FailedToLoadYamlFile,
    YamlDecodeError {
        ty: String,
        version: String,
        kind: String,
    },
    UnrecognizedObject,
}

impl From<std::io::Error> for KorrecteError {
    fn from(e: std::io::Error) -> Self {
        KorrecteError::Io(e)
    }
}

impl From<toml::de::Error> for KorrecteError {
    fn from(e: toml::de::Error) -> Self {
        KorrecteError::Serde(e)
    }
}

impl From<::kube::Error> for KorrecteError {
    fn from(e: ::kube::Error) -> Self {
        KorrecteError::KubeConfig(e)
    }
}
