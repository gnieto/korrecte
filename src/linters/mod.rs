use kube::{
    api::{Api, Informer, WatchEvent, Object},
    client::APIClient,
    config,
};
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};

pub(crate) mod required_labels;

pub trait Lint {
    fn spec(&self) -> LintSpec;

    fn pod(&self, pod: &Object<PodSpec, PodStatus>) {}
}

pub enum Group {
    Audit,
}

pub struct LintSpec {
    group: Group,
}

