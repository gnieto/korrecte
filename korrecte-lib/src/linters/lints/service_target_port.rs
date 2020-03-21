use crate::linters::Lint;

use crate::f;
use crate::linters::evaluator::Context;
use crate::reporting::Finding;
use crate::visitor::{visit_all_pod_specs, PodSpecVisitor};
use k8s_openapi::api::core::v1::PodSpec;
use k8s_openapi::api::core::v1::Service;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use std::collections::BTreeMap;

const LINT_NAME: &str = "service_target_port";

pub(crate) struct ServiceTargetPort;

impl Lint for ServiceTargetPort {
    fn name(&self) -> &str {
        LINT_NAME
    }

    fn core_v1_service(&self, service: &Service, context: &Context) {

    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{analyze_file, filter_findings_by};
    use std::path::Path;

    #[test]
    fn it_finds_services_without_matching_labels() {
        let findings = analyze_file(Path::new("../tests/service_without_matching_labels.yml"));
        let findings = filter_findings_by(findings, super::LINT_NAME);

        assert_eq!(2, findings.len());
        assert_eq!("my-service", findings[0].name());
        assert_eq!("multi-tag-non-match", findings[1].name());
    }
}
