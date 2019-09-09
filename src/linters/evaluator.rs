use crate::linters::LintList;
use crate::kube::NewObjectRepository;
use crate::reporting::{Reporter, Finding};
use crate::linters::KubeObjectType;

pub struct OneShotEvaluator;

impl OneShotEvaluator {
    pub fn evaluate(reporter: &dyn Reporter, list: LintList, object_repository: &Box<dyn NewObjectRepository>) {
        for lint in list.iter() {
            for object in object_repository.all() {
                match object {
                    &KubeObjectType::V1Pod(ref o) => {
                        Self::report_lints(reporter, lint.v1_pod(o));
                    },
                    &KubeObjectType::V1Node(ref o) => {
                        Self::report_lints(reporter, lint.v1_node(o));
                    },
                    &KubeObjectType::V1Service(ref o) => {
                        Self::report_lints(reporter, lint.v1_service(o));
                    },
                    &KubeObjectType::V1Deployment(ref o) => {
                        Self::report_lints(reporter, lint.v1_deployment(o));
                    },
                    _ => {}
                }
            }
        }
    }

    fn report_lints(reporter: &dyn Reporter, findings: Vec<Finding>) {
        for f in findings {
            reporter.report(f);
        }
    }
}