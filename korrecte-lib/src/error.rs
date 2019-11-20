use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub enum KorrecteError {
    Io(std::io::Error),
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

impl Display for KorrecteError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            KorrecteError::Io(e) => write!(f, "Error performing an IO operation: {}", e),
            KorrecteError::Generic(e) => write!(f, "{}", e),
            KorrecteError::FailedToLoadYamlFile => write!(f, "Could not load YAML file"),
            KorrecteError::YamlDecodeError { ty, version, kind } => {
                write!(f, "Could not decode YAML file: {} {} {}", ty, version, kind)
            }
            KorrecteError::UnrecognizedObject => write!(f, "Unrecognized Kubernetes object"),
        }
    }
}
