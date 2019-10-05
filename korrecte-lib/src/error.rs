use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub enum KorrecteError {
    Io(std::io::Error),
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

impl From<::kube::Error> for KorrecteError {
    fn from(e: ::kube::Error) -> Self {
        KorrecteError::KubeConfig(e)
    }
}

impl Display for KorrecteError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            KorrecteError::Io(e) => write!(f, "Error performing an IO operation: {}", e),
            KorrecteError::KubeConfig(e) => write!(f, "Error loading kubeconfig: {}", e),
            KorrecteError::Generic(e) => write!(f, "{}", e),
            KorrecteError::FailedToLoadYamlFile => write!(f, "Could not load YAML file"),
            KorrecteError::YamlDecodeError { ty, version, kind } => {
                write!(f, "Could not decode YAML file: {} {} {}", ty, version, kind)
            }
            KorrecteError::UnrecognizedObject => write!(f, "Unrecognized Kubernetes object"),
        }
    }
}
