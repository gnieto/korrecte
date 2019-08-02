use crate::linters::LintList;
use crate::kube::objects::ObjectRepository;
use crate::reporting::Reporter;

pub struct OneShotEvaluator;

impl OneShotEvaluator {
    pub fn evaluate(reporter: &dyn Reporter, list: LintList, object_repository: ObjectRepository) {
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