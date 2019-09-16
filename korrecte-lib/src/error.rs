#[derive(Debug)]
#[allow(unused)]
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
