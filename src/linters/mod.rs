use kube::api::Object;
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};

pub(crate) mod required_labels;
pub(crate) mod overlapping_probes;

pub trait Lint {
    fn spec(&self) -> LintSpec;

    fn pod(&self, _pod: &Object<PodSpec, PodStatus>) {}
}

#[derive(Clone)]
pub enum Group {
    Audit,
    Configuration,
}

#[derive(Clone)]
pub struct LintSpec {
    pub group: Group,
    pub name: String,
}