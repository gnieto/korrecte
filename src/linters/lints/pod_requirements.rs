use crate::linters::{Group, KubeObjectType, Lint, LintSpec};

use crate::reporting::Finding;
use crate::visitor::{pod_spec_visit, PodSpecVisitor};
use k8s_openapi::api::core::v1::Container;
use k8s_openapi::api::core::v1::PodSpec;
use kube::api::ObjectMeta;
use std::collections::BTreeMap;

/// **What it does:** Checks for pods without resource limits
///
/// **Why is this bad?** Pods without resource limits may provoke a denial-of-service of the
/// processes running on the same node.
///
/// **Known problems:** Memory limits are hard-requirements and processes will be OOM killed if they
/// go beyond the limits. CPU limits may cause inexplicable pauses due to CFS (Completely Fair Scheduler)
///
/// **References**
///
#[derive(Default)]
pub(crate) struct PodRequirements;

impl Lint for PodRequirements {
    fn object(&self, object: &KubeObjectType) -> Vec<Finding> {
        let mut visitor = PodRequirementsVisitor::default();
        pod_spec_visit(&object, &mut visitor);

        visitor.findings
    }
}

impl PodRequirements {
    fn spec() -> LintSpec {
        LintSpec {
            group: Group::Security,
            name: "pod_requirements".to_string(),
        }
    }
}

#[derive(Default)]
struct PodRequirementsVisitor {
    findings: Vec<Finding>,
}

impl PodSpecVisitor for PodRequirementsVisitor {
    fn visit_pod_spec(&mut self, pod_spec: &PodSpec, meta: &ObjectMeta) {
        self.check_pod_spec(pod_spec, meta);
    }
}

impl PodRequirementsVisitor {
    fn check_pod_spec(&mut self, pod_spec: &PodSpec, metadata: &ObjectMeta) {
        for container in pod_spec.containers.iter() {
            self.check_container(container, metadata);
        }
    }

    fn check_container(&mut self, container: &Container, metadata: &ObjectMeta) {
        match container.resources {
            None => {
                self.missing_cpu_limit(metadata, container);
                self.missing_mem_limit(metadata, container);
                self.missing_cpu_requirement(metadata, container);
                self.missing_mem_requirement(metadata, container);
            }
            Some(ref req) => {
                let empty_map = BTreeMap::new();

                // TODO: Move into dedicated methods
                // Check limits
                let limits = req.limits.as_ref().unwrap_or(&empty_map);
                if !limits.contains_key("cpu") {
                    self.missing_cpu_limit(metadata, container);
                }
                if !limits.contains_key("memory") {
                    self.missing_mem_limit(metadata, container);
                }

                // Check requirements
                let requests = req.requests.as_ref().unwrap_or(&empty_map);
                if !requests.contains_key("cpu") {
                    self.missing_cpu_requirement(metadata, container);
                }
                if !requests.contains_key("memory") {
                    self.missing_mem_requirement(metadata, container);
                }
            }
        }
    }

    fn missing_cpu_limit(&mut self, metadata: &ObjectMeta, container: &Container) {
        self.missing_resource(metadata, container, "missing_cpu_limit");
    }

    fn missing_mem_limit(&mut self, metadata: &ObjectMeta, container: &Container) {
        self.missing_resource(metadata, container, "missing_mem_limit");
    }

    fn missing_cpu_requirement(&mut self, metadata: &ObjectMeta, container: &Container) {
        self.missing_resource(metadata, container, "missing_cpu_requirement");
    }

    fn missing_mem_requirement(&mut self, metadata: &ObjectMeta, container: &Container) {
        self.missing_resource(metadata, container, "missing_mem_requirement");
    }

    fn missing_resource(&mut self, metadata: &ObjectMeta, container: &Container, key: &str) {
        let finding = Finding::new(PodRequirements::spec(), metadata.clone())
            .add_metadata(key, "")
            .add_metadata("container", container.name.clone());

        self.findings.push(finding);
    }
}

#[cfg(test)]
mod tests {
    use crate::linters::lints::pod_requirements::PodRequirements;
    use crate::tests::{analyze_file, filter_findings_by};
    use std::path::Path;

    #[test]
    pub fn it_does_not_find_anything_on_properly_configured_pod() {
        let findings = analyze_file(Path::new("tests/pod_requirements.yaml"));
        let findings = filter_findings_by(findings, &PodRequirements::spec());

        assert_eq!(0, findings.len());
    }

    #[test]
    pub fn it_finds_pods_with_missing_requirements_or_limits() {
        let findings = analyze_file(Path::new("tests/pod_requirements_ko.yaml"));
        let findings = filter_findings_by(findings, &PodRequirements::spec());

        assert_eq!(8, findings.len());
    }
}
