use crate::linters::lints::environment_passwords::Config as EnvironmentPasswordsConfig;
use crate::linters::lints::required_labels::Config as RequiredLabelsConfig;
use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub(crate) korrecte: KorrecteConfig,

    #[serde(default)]
    pub(crate) required_labels: RequiredLabelsConfig,

    #[serde(default)]
    pub(crate) environment_passwords: EnvironmentPasswordsConfig,
}

#[derive(Default, Deserialize, Debug)]
pub struct KorrecteConfig {
    pub(crate) allowed_namespaces: Vec<String>,
    pub(crate) ignored_namespaces: Vec<String>,
}
