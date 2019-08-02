use serde::Deserialize;
use crate::linters::lints::required_labels::Config as RequiredLabelsConfig;
use crate::linters::lints::environment_passwords::Config as EnvironmentPasswordsConfig;

#[derive(Default, Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub(crate) required_labels: RequiredLabelsConfig,
    #[serde(default)]
    pub(crate) environment_var: EnvironmentPasswordsConfig,
}