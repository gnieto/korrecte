use crate::linters::LintList;
use crate::kube::ObjectRepository;
use crate::reporting::Reporter;

pub struct OneShotEvaluator;

impl OneShotEvaluator {
    pub fn evaluate<O: ObjectRepository>(reporter: &dyn Reporter, list: LintList, object_repository: O) {
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
    }
}