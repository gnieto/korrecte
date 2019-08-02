use kube::api::Object;
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use k8s_openapi::api::core::v1::{ServiceSpec, ServiceStatus};
use crate::config::Config;
use crate::reporting::Reporter;
use crate::linters;
use crate::kube::objects::ObjectRepository;

pub(crate) mod lints;
pub(crate) mod evaluator;

pub trait Lint {
    fn spec(&self) -> LintSpec;

    fn pod(&self, _pod: &Object<PodSpec, PodStatus>) {}
    fn service(&self, _svc: &Object<ServiceSpec, ServiceStatus>) {}
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
    pub fn all<'a, R: Reporter + Clone + 'a>(cfg: Config, reporter: R, object_repository: ObjectRepository) -> LintList<'a> {
        let required = linters::lints::required_labels::RequiredLabels::new(cfg.required_labels.clone(), reporter.clone());
        let overlapping = linters::lints::overlapping_probes::OverlappingProbes::new(reporter.clone());
        let never = linters::lints::never_restart_with_liveness_probe::NeverRestartWithLivenessProbe::new(reporter.clone());
        let service_labels = linters::lints::service_without_matching_labels::ServiceWithoutMatchingLabels::new(reporter.clone(), object_repository.clone());
        let passwords = linters::lints::environment_passwords::EnvironmentPasswords::new(reporter, cfg.environment_var.clone());

        vec![
            Box::new(required),
            Box::new(overlapping),
            Box::new(never),
            Box::new(service_labels),
            Box::new(passwords),
        ]
    }
}