use crate::linters::{Group, Lint, LintSpec};

use crate::reporting::Finding;
use k8s_openapi::api::apps::v1::{DeploymentSpec, DeploymentStatus};
use k8s_openapi::api::core::v1::Container;
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use kube::api::KubeObject;
use kube::api::{Object, ObjectMeta};
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
    // TODO: Add a visitor which visits PodSpecs for all kind of objects
    // TODO: Add tests
    fn v1_pod(&self, pod: &Object<PodSpec, PodStatus>) -> Vec<Finding> {
        let mut findings = Vec::new();

        self.check_pod_spec(&mut findings, &pod.spec, &pod.meta());

        findings
    }

    fn v1_deployment(&self, deploy: &Object<DeploymentSpec, DeploymentStatus>) -> Vec<Finding> {
        let mut findings = Vec::new();

        if let Some(ref pod_spec) = deploy.spec.template.spec {
            self.check_pod_spec(&mut findings, &pod_spec, &deploy.meta());
        }

        findings
    }

    fn spec(&self) -> LintSpec {
        LintSpec {
            group: Group::Security,
            name: "pod_requirements".to_string(),
        }
    }
}

impl PodRequirements {
    fn check_pod_spec(
        &self,
        findings: &mut Vec<Finding>,
        pod_spec: &PodSpec,
        metadata: &ObjectMeta,
    ) {
        for container in pod_spec.containers.iter() {
            self.check_container(findings, container, metadata);
        }
    }

    fn check_container(
        &self,
        findings: &mut Vec<Finding>,
        container: &Container,
        metadata: &ObjectMeta,
    ) {
        match container.resources {
            None => {
                self.missing_cpu_limit(findings, metadata, container);
                self.missing_mem_limit(findings, metadata, container);
                self.missing_cpu_requirement(findings, metadata, container);
                self.missing_mem_requirement(findings, metadata, container);
            }
            Some(ref req) => {
                let empty_map = BTreeMap::new();

                // TODO: Move into dedicated methods
                // Check limits
                let limits = req.limits.as_ref().unwrap_or(&empty_map);
                if !limits.contains_key("cpu") {
                    self.missing_cpu_limit(findings, metadata, container);
                }
                if !limits.contains_key("memory") {
                    self.missing_mem_limit(findings, metadata, container);
                }

                // Check requirements
                let requests = req.requests.as_ref().unwrap_or(&empty_map);
                if !requests.contains_key("cpu") {
                    self.missing_cpu_requirement(findings, metadata, container);
                }
                if !requests.contains_key("memory") {
                    self.missing_mem_requirement(findings, metadata, container);
                }
            }
        }
    }

    fn missing_cpu_limit(
        &self,
        findings: &mut Vec<Finding>,
        metadata: &ObjectMeta,
        container: &Container,
    ) {
        self.missing_resource(findings, metadata, container, "missing_cpu_limit");
    }

    fn missing_mem_limit(
        &self,
        findings: &mut Vec<Finding>,
        metadata: &ObjectMeta,
        container: &Container,
    ) {
        self.missing_resource(findings, metadata, container, "missing_mem_limit");
    }

    fn missing_cpu_requirement(
        &self,
        findings: &mut Vec<Finding>,
        metadata: &ObjectMeta,
        container: &Container,
    ) {
        self.missing_resource(findings, metadata, container, "missing_cpu_requirement");
    }

    fn missing_mem_requirement(
        &self,
        findings: &mut Vec<Finding>,
        metadata: &ObjectMeta,
        container: &Container,
    ) {
        self.missing_resource(findings, metadata, container, "missing_mem_requirement");
    }

    fn missing_resource(
        &self,
        findings: &mut Vec<Finding>,
        metadata: &ObjectMeta,
        container: &Container,
        key: &str,
    ) {
        let finding = Finding::new(self.spec().clone(), metadata.clone())
            .add_metadata(key, "")
            .add_metadata("container", container.name.clone());

        findings.push(finding);
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{analyze_file, filter_findings_by};
    use std::path::Path;

    #[test]
    pub fn it_does_not_find_anything_on_properly_configured_pod() {
        let findings = analyze_file(Path::new("tests/pod_requirements.yaml"));
        let findings = filter_findings_by(findings, "pod_requirements");

        assert_eq!(0, findings.len());
    }

    #[test]
    pub fn it_finds_pods_with_missing_requirements_or_limits() {
        let findings = analyze_file(Path::new("tests/pod_requirements_ko.yaml"));
        let findings = filter_findings_by(findings, "pod_requirements");

        assert_eq!(4, findings.len());
    }
}
