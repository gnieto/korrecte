use crate::config::Config;
use crate::linters;
use anyhow::*;
use serde::{Deserialize, Serialize};

pub mod evaluator;
mod lint;
pub(crate) mod lints;
pub use lint::{KubeObjectType, Lint};
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Group {
    Audit,
    Configuration,
    Security,
}

impl ToString for Group {
    fn to_string(&self) -> String {
        match self {
            Group::Audit => "audit".to_string(),
            Group::Configuration => "configuration".to_string(),
            Group::Security => "security".to_string(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct LintSpec {
    pub group: Group,
    pub name: String,
    pub description: String,
    pub references: Vec<String>,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
struct LintCfg {
    specs: Vec<LintSpec>,
}

pub struct LintSpecLoader {
    specs: HashMap<String, LintSpec>,
}

impl LintSpecLoader {
    pub fn new() -> Result<LintSpecLoader> {
        let yaml = include_str!("../../../lints.yaml");
        let cfg: LintCfg = serde_yaml::from_str(yaml)?;
        let mut lint_def = HashMap::new();

        for lc in cfg.specs {
            lint_def.insert(lc.name.clone(), lc);
        }

        Ok(LintSpecLoader { specs: lint_def })
    }

    pub fn get(&self, name: &str) -> Option<&LintSpec> {
        self.specs.get(name)
    }

    pub fn all(&self) -> &HashMap<String, LintSpec> {
        &self.specs
    }
}

pub type LintList<'a> = Vec<Box<dyn Lint + 'a>>;

pub struct LintCollection;

impl LintCollection {
    pub fn all<'a>(cfg: Config) -> LintList<'a> {
        let alb_ingress = linters::lints::alb_ingress_instance::AlbIngressInstance {};
        let passwords = linters::lints::environment_passwords::EnvironmentPasswords::new(
            cfg.environment_passwords.clone(),
        );
        let never = linters::lints::never_restart_with_liveness_probe::NeverRestartWithLivenessProbe::default();
        let overlapping = linters::lints::overlapping_probes::OverlappingProbes::default();
        let pdb_min = linters::lints::pdb_min_replicas::PdbMinReplicas {};
        let pod_requirements = linters::lints::pod_requirements::PodRequirements::default();
        let required =
            linters::lints::required_labels::RequiredLabels::new(cfg.required_labels.clone());
        let role_similar = linters::lints::role_similar_names::RoleSimilarNames {};
        let service_labels =
            linters::lints::service_without_matching_labels::ServiceWithoutMatchingLabels {};
        let statefulset_grace_period_zero =
            linters::lints::statefulset_grace_period_zero::StatefulsetGracePeriodZero::default();

        vec![
            Box::new(alb_ingress),
            Box::new(passwords),
            Box::new(never),
            Box::new(overlapping),
            Box::new(pdb_min),
            Box::new(pod_requirements),
            Box::new(required),
            Box::new(role_similar),
            Box::new(service_labels),
            Box::new(statefulset_grace_period_zero),
        ]
    }
}
