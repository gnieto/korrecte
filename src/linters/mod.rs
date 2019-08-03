use kube::api::Object;
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use k8s_openapi::api::core::v1::{ServiceSpec, ServiceStatus};
use crate::config::Config;
use crate::reporting::Reporter;
use crate::linters;
use crate::kube::ObjectRepository;

pub(crate) mod lints;
pub(crate) mod evaluator;

pub trait Lint {
    fn spec(&self) -> LintSpec;

    fn pod(&self, _pod: &Object<PodSpec, PodStatus>, _reporter: &dyn Reporter) {}
    fn service(&self, _svc: &Object<ServiceSpec, ServiceStatus>, _reporter: &dyn Reporter) {}
}

#[derive(Clone)]
pub enum Group {
    Audit,
    Configuration,
    Security,
}

#[derive(Clone)]
pub struct LintSpec {
    pub group: Group,
    pub name: String,
}

pub type LintList<'a> = Vec<Box<dyn Lint + 'a>>;

pub struct LintCollection;

impl LintCollection {
    pub fn all<'a, O: ObjectRepository + Clone + 'a>(cfg: Config, object_repository: O) -> LintList<'a> {
        let required = linters::lints::required_labels::RequiredLabels::new(cfg.required_labels.clone());
        let overlapping = linters::lints::overlapping_probes::OverlappingProbes::default();
        let never = linters::lints::never_restart_with_liveness_probe::NeverRestartWithLivenessProbe::default();
        let service_labels = linters::lints::service_without_matching_labels::ServiceWithoutMatchingLabels::new(object_repository.clone());
        let passwords = linters::lints::environment_passwords::EnvironmentPasswords::new(cfg.environment_passwords.clone());

        vec![
            Box::new(required),
            Box::new(overlapping),
            Box::new(never),
            Box::new(service_labels),
            Box::new(passwords),
        ]
    }
}