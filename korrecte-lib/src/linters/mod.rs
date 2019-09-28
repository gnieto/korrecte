use crate::config::Config;
use crate::kube::ObjectRepository;
use crate::linters;
use serde::{Serialize, Deserialize};

#[cfg(feature = "wasm")]
mod wasm;

pub(crate) mod evaluator;
mod lint;
pub(crate) mod lints;
pub use lint::{KubeObjectType, Lint};
pub use evaluator::OneShotEvaluator;
// TODO: Remove; Retrocompatibility
pub use korrecte_shared::reporting::{Group, LintSpec};

pub type LintList<'a> = Vec<Box<dyn Lint + 'a>>;

pub struct LintCollection;

impl LintCollection {
    pub fn all(cfg: Config, object_repository: &dyn ObjectRepository) -> LintList {
        let required =
            linters::lints::required_labels::RequiredLabels::new(cfg.required_labels.clone());
        let overlapping = linters::lints::overlapping_probes::OverlappingProbes::default();
        let never = linters::lints::never_restart_with_liveness_probe::NeverRestartWithLivenessProbe::default();
        let service_labels =
            linters::lints::service_without_matching_labels::ServiceWithoutMatchingLabels::new(
                object_repository,
            );
        let passwords = linters::lints::environment_passwords::EnvironmentPasswords::new(
            cfg.environment_passwords.clone(),
        );
        let pdb_min = linters::lints::pdb_min_replicas::PdbMinReplicas::new(object_repository);
        let statefulset_grace_period_zero =
            linters::lints::statefulset_grace_period_zero::StatefulsetGracePeriodZero::default();
        let pod_requirements = linters::lints::pod_requirements::PodRequirements::default();

        vec![
            Box::new(required),
            Box::new(overlapping),
            Box::new(never),
            Box::new(service_labels),
            Box::new(passwords),
            Box::new(pdb_min),
            Box::new(statefulset_grace_period_zero),
            Box::new(pod_requirements),
        ]
    }
}
