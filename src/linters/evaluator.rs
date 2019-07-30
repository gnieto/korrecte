use crate::linters::LintList;
use crate::kube::objects::ObjectRepository;

pub struct OneShotEvaluator;

impl OneShotEvaluator {
    pub fn evaluate(list: LintList, object_repository: ObjectRepository) {
        for pod in object_repository.pods().iter() {
            for lint in list.iter() {
                lint.pod(pod);
            }
        }

        for svc in object_repository.services().iter() {
            for lint in list.iter() {
                lint.service(svc);
            }
        }
    }
}