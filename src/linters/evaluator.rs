use crate::linters::LintList;
use crate::kube::ObjectRepository;
use crate::reporting::Reporter;

pub struct OneShotEvaluator;

impl OneShotEvaluator {
    pub fn evaluate(reporter: &dyn Reporter, list: LintList, object_repository: &Box<dyn ObjectRepository>) {
        for pod in object_repository.pods().iter() {
            for lint in list.iter() {
                lint.pod(pod, reporter);
            }
        }

        for svc in object_repository.services().iter() {
            for lint in list.iter() {
                lint.service(svc, reporter);
            }
        }

        for pdb in object_repository.pod_disruption_budgets().iter() {
            for lint in list.iter() {
                lint.pod_disruption_budget(pdb, reporter);
            }
        }
    }
}