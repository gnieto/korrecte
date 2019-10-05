#[derive(Debug)]
pub enum CliError {
    MissingPath,
    KorrecteError(korrecte::error::KorrecteError),
    Io(std::io::Error),
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
