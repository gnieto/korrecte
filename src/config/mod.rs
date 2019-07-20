use serde::Deserialize;
use crate::linters::required_labels::Config as RequiredLabelsConfig;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub(crate) required_labels: RequiredLabelsConfig,
}