use crate::linters::{Group, KubeObjectType, Lint, LintSpec};

use crate::reporting::Finding;
use crate::visitor::{pod_spec_visit, PodSpecVisitor};
use k8s_openapi::api::core::v1::{Container, PodSpec, Probe};
use kube::api::ObjectMeta;
use std::time::Duration;

/// **What it does:** Finds pods which liveness probe *may* execute before all readiness probes has
/// been executed
///
/// **Why is this bad?** Readiness probe is used to determine if the container is ready while liveness
/// probe is used to determine if the application is alive.
/// Executing a liveness probe *before* the container is ready will provoke that pod change the
/// status to failed.
///
/// **Known problems:** One may assume that liveness probes starts being tested once a container
/// changes the status to `Ready`, but this is not the case (see references)
///
/// **References:**
/// - https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#container-probes
/// - https://github.com/kubernetes/kubernetes/issues/27114
/// - https://cloud.google.com/blog/products/gcp/kubernetes-best-practices-setting-up-health-checks-with-readiness-and-liveness-probes
#[derive(Default)]
pub(crate) struct OverlappingProbes;

impl Lint for OverlappingProbes {
    fn object(&self, object: &KubeObjectType) -> Vec<Finding> {
        let mut visitor = OverlappingProbesVisitor::default();
        pod_spec_visit(&object, &mut visitor);

        visitor.findings
    }
}

#[derive(Default)]
struct OverlappingProbesVisitor {
    findings: Vec<Finding>,
}

impl PodSpecVisitor for OverlappingProbesVisitor {
    fn visit_pod_spec(&mut self, pod_spec: &PodSpec, meta: &ObjectMeta) {
        for c in pod_spec.containers.iter() {
            self.check_container_probes(&c, &meta);
        }
    }
}

impl OverlappingProbesVisitor {
    fn check_container_probes(&mut self, c: &Container, object_meta: &ObjectMeta) {
        let readiness_probe = c.readiness_probe.as_ref().map(Self::calculate_time_frame);
        let liveness_probes = c.liveness_probe.as_ref().map(Self::calculate_time_frame);

        if let (Some(readiness), Some(liveness)) = (readiness_probe, liveness_probes) {
            if readiness.overlaps_with(&liveness) {
                let readiness_end = format!("{:?}", readiness.end);
                let liveness_start = format!("{:?}", liveness.start);

                let finding = Finding::new(OverlappingProbes::spec(), object_meta.clone())
                    .add_metadata("container".to_string(), c.name.clone())
                    .add_metadata("readiness_max_delay".to_string(), readiness_end)
                    .add_metadata("liveness_start".to_string(), liveness_start);

                self.findings.push(finding);
            }
        }
    }

    fn calculate_time_frame(probe: &Probe) -> TimeFrame {
        let initial_delay = probe.initial_delay_seconds.unwrap_or(0) as u64;
        let initial = Duration::new(initial_delay, 0);

        let failure_threshold = probe.failure_threshold.unwrap_or(0) as u64;
        let success_threshold = probe.success_threshold.unwrap_or(0) as u64;
        let max_probe_amount = failure_threshold.saturating_add(success_threshold);

        let timeout = probe.timeout_seconds.unwrap_or(1) as u64;
        let max_delay = max_probe_amount.saturating_mul(timeout);
        let max_duration = initial
            .checked_add(Duration::new(max_delay, 0))
            .unwrap_or_else(|| initial);

        TimeFrame {
            start: initial,
            end: max_duration,
        }
    }
}

impl OverlappingProbes {
    fn spec() -> LintSpec {
        LintSpec {
            group: Group::Configuration,
            name: "overlapping_probes".to_string(),
        }
    }
}

struct TimeFrame {
    start: Duration,
    end: Duration,
}

impl TimeFrame {
    pub fn overlaps_with(&self, frame: &TimeFrame) -> bool {
        self.end > frame.start
    }
}

#[cfg(test)]
mod tests {
    use crate::linters::lints::overlapping_probes::OverlappingProbes;
    use crate::tests::{analyze_file, filter_findings_by};
    use std::path::Path;

    #[test]
    fn it_finds_never_restart_errors() {
        let findings = analyze_file(Path::new("tests/overlapping_probes.yaml"));
        let findings = filter_findings_by(findings, &OverlappingProbes::spec());

        assert_eq!(1, findings.len());
        assert_eq!(findings[0].object_metadata().name, "hello-node");
    }
}
