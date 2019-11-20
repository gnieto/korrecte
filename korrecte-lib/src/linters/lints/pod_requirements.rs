use crate::linters::{Group, KubeObjectType, Lint, LintSpec};

use crate::linters::evaluator::Context;
use crate::reporting::Finding;
use crate::visitor::{pod_spec_visit, PodSpecVisitor};
use k8s_openapi::api::core::v1::Container;
use k8s_openapi::api::core::v1::PodSpec;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
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
    fn object(&self, object: &KubeObjectType, context: &Context) {
        let mut visitor = PodRequirementsVisitor { context };
        pod_spec_visit(&object, &mut visitor);
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

struct PodRequirementsVisitor<'a> {
    context: &'a Context<'a>,
}

impl<'a> PodSpecVisitor for PodRequirementsVisitor<'a> {
    fn visit_pod_spec(&mut self, pod_spec: &PodSpec, meta: Option<&ObjectMeta>) {
        self.check_pod_spec(pod_spec, meta);
    }
}

impl<'a> PodRequirementsVisitor<'a> {
    fn check_pod_spec(&self, pod_spec: &PodSpec, metadata: Option<&ObjectMeta>) {
        for container in pod_spec.containers.iter() {
            self.check_container(container, metadata);
        }
    }

    fn check_container(&self, container: &Container, metadata: Option<&ObjectMeta>) {
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

    fn missing_cpu_limit(&self, metadata: Option<&ObjectMeta>, container: &Container) {
        self.missing_resource(metadata, container, "missing_cpu_limit");
    }

    fn missing_mem_limit(&self, metadata: Option<&ObjectMeta>, container: &Container) {
        self.missing_resource(metadata, container, "missing_mem_limit");
    }

    fn missing_cpu_requirement(&self, metadata: Option<&ObjectMeta>, container: &Container) {
        self.missing_resource(metadata, container, "missing_cpu_requirement");
    }

    fn missing_mem_requirement(&self, metadata: Option<&ObjectMeta>, container: &Container) {
        self.missing_resource(metadata, container, "missing_mem_requirement");
    }

    fn missing_resource(&self, metadata: Option<&ObjectMeta>, container: &Container, key: &str) {
        let finding = Finding::new(PodRequirements::spec(), metadata.cloned())
            .add_metadata(key, "")
            .add_metadata("container", container.name.clone());

        self.context.reporter.report(finding);
    }
}

#[cfg(test)]
mod tests {
    use crate::linters::lints::pod_requirements::PodRequirements;
    use crate::tests::{analyze_file, filter_findings_by};
    use std::path::Path;

    #[test]
    pub fn it_does_not_find_anything_on_properly_configured_pod() {
        let findings = analyze_file(Path::new("../tests/pod_requirements.yaml"));
        let findings = filter_findings_by(findings, &PodRequirements::spec());

        assert_eq!(0, findings.len());
    }

    #[test]
    pub fn it_finds_pods_with_missing_requirements_or_limits() {
        let findings = analyze_file(Path::new("../tests/pod_requirements_ko.yaml"));
        let findings = filter_findings_by(findings, &PodRequirements::spec());

        assert_eq!(8, findings.len());
    }
}
