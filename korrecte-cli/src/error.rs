#[derive(Debug)]
pub enum CliError {
    ConfigLoadError,
    KorrecteError(korrecte::error::KorrecteError),
    Io(std::io::Error),
}

impl From<toml::de::Error> for CliError {
    fn from(_e: toml::de::Error) -> Self {
        CliError::ConfigLoadError
    }
}

impl From<korrecte::error::KorrecteError> for CliError {
    fn from(e: korrecte::error::KorrecteError) -> Self {
        CliError::KorrecteError(e)
    }
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::Io(e)
    }
}
