use crate::linters::lints::environment_passwords::Config as EnvironmentPasswordsConfig;
use crate::linters::lints::required_labels::Config as RequiredLabelsConfig;
use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub(crate) required_labels: RequiredLabelsConfig,

    #[serde(default)]
    pub(crate) environment_passwords: EnvironmentPasswordsConfig,
}
