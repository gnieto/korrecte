use super::{Lint, LintSpec, Group};

use kube::api::{Object, ObjectMeta};
use k8s_openapi::api::core::v1::{PodSpec, PodStatus, Probe, Container};
use crate::reporting::{Reporter, Finding};
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
pub(crate) struct OverlappingProbes<R: Reporter> {
    reporter: R,
}

impl<R: Reporter> Lint for OverlappingProbes<R> {
    fn spec(&self) -> LintSpec {
        LintSpec {
            group: Group::Configuration,
            name: "overlapping_probes".to_string(),
        }
    }

    fn pod(&self, pod: &Object<PodSpec, PodStatus>) {
        for c in pod.spec.containers.iter() {
            self.check_container_probes(&c, &pod.metadata);
        }
    }
}


impl<R: Reporter> OverlappingProbes<R> {
    pub fn new(reporter: R) -> Self {
        OverlappingProbes {
            reporter,
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
            .unwrap_or(initial.clone());

        TimeFrame {
            start: initial,
            end: max_duration,
        }
    }

    fn check_container_probes(&self, c: &Container, object_meta: &ObjectMeta) {
        let readiness_probe = c.readiness_probe.as_ref().map(Self::calculate_time_frame);
        let liveness_probes = c.liveness_probe.as_ref().map(Self::calculate_time_frame);

        if let (Some(readiness), Some(liveness)) = (readiness_probe, liveness_probes) {
            if readiness.overlaps_with(&liveness) {
                let readiness_end = format!("{:?}", readiness.end);
                let liveness_start = format!("{:?}", liveness.start);

                let finding = Finding::new(self.spec().clone(), object_meta.clone())
                    .add_metadata("container".to_string(), c.name.clone())
                    .add_metadata("readiness_max_delay".to_string(), readiness_end)
                    .add_metadata("liveness_start".to_string(), liveness_start);

                self.reporter.report(finding);
            }
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
