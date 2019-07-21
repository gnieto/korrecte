use serde::Deserialize;
use crate::linters::required_labels::Config as RequiredLabelsConfig;

#[derive(Default, Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub(crate) required_labels: RequiredLabelsConfig,
}