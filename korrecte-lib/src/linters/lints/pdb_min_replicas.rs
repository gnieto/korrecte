use crate::linters::{Group, KubeObjectType, Lint, LintSpec};

use crate::kube::ObjectRepository;
use crate::linters::evaluator::Context;
use crate::reporting::Finding;
use crate::{f, m};
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler;
use k8s_openapi::api::policy::v1beta1::PodDisruptionBudget;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use std::borrow::Borrow;

/// **What it does:** Checks that pod controllers associated to a pod disruption budget has at least one more replica
/// than PDB min_unavailable
///
/// **Why is this bad?** The pod controller won't be able to be rolled out, as no pod can be evicted (as min_unavailable is >= to the amount of replicas desired). This may cause
/// that a node can not be cordoned.
///
/// **Known problems:** None
///
/// **References**
/// - https://itnext.io/kubernetes-in-production-poddisruptionbudget-1380009aaede
pub(crate) struct PdbMinReplicas;

impl Lint for PdbMinReplicas {
    fn policy_v1beta1_pod_disruption_budget(&self, pdb: &PodDisruptionBudget, context: &Context) {
        if let Some(pdb_min_available) = Self::get_min_replicas(pdb) {
            self.matching_deployments(pdb, context, pdb_min_available);
            self.matching_hpas(pdb, context, pdb_min_available);
        }
    }
}

impl PdbMinReplicas {
    fn spec() -> LintSpec {
        LintSpec {
            group: Group::Configuration,
            name: "pdb_min_replicas".to_string(),
        }
    }

    fn get_min_replicas(pdb: &PodDisruptionBudget) -> Option<i32> {
        if let Some(IntOrString::Int(amount)) = f!(pdb.spec, max_unavailable) {
            return Some(*amount);
        }

        None
    }

    fn find_matching_deployments<'a>(
        &'a self,
        object_repository: &'a dyn ObjectRepository,
        pdb: &PodDisruptionBudget,
    ) -> Vec<&'a Deployment> {
        object_repository
            .iter()
            .filter_map(|o| match o {
                KubeObjectType::AppsV1Deployment(d) => Some(d),
                _ => None,
            })
            .filter(|deploy| Self::deploy_matches_with_pdb(pdb, deploy))
            .map(|o| o.borrow())
            .collect()
    }

    fn find_matching_hpa<'a>(
        &'a self,
        object_repository: &'a dyn ObjectRepository,
        pdb: &PodDisruptionBudget,
    ) -> Vec<&'a HorizontalPodAutoscaler> {
        object_repository
            .iter()
            .filter_map(|o| match o {
                KubeObjectType::AutoscalingV1HorizontalPodAutoscaler(hpa) => Some(hpa),
                _ => None,
            })
            .filter(|hpa| {
                let kind = m!(hpa.spec, scale_target_ref, kind);
                if let Some("Deployment") = kind.map(|r| r.as_str()) {
                    // Check if there's any target deployment which is targeted by the PDB
                    object_repository
                        .iter()
                        .filter_map(|object| match object {
                            KubeObjectType::AppsV1Deployment(d) => Some(d),
                            _ => None,
                        })
                        .any(|deploy| Self::deploy_matches_with_pdb(pdb.borrow(), &deploy))
                } else {
                    false
                }
            })
            .map(|o| o.borrow())
            .collect()
    }

    fn matching_deployments(
        &self,
        pdb: &PodDisruptionBudget,
        context: &Context,
        pdb_min_available: i32,
    ) {
        let matching_deployments = self.find_matching_deployments(context.repository, pdb);

        matching_deployments.iter().for_each(|d| {
            let deploy_replicas = *f!(d.spec, replicas).unwrap_or(&0);

            if pdb_min_available >= deploy_replicas {
                let finding = Finding::new(Self::spec(), pdb.metadata.clone())
                    .add_metadata("deploy_replicas", deploy_replicas)
                    .add_metadata("pdb_min_available", pdb_min_available.to_string());
                context.reporter.report(finding);
            }
        })
    }

    fn matching_hpas(&self, pdb: &PodDisruptionBudget, context: &Context, pdb_min_available: i32) {
        let matching_hpa = self.find_matching_hpa(context.repository, pdb);

        matching_hpa.iter().for_each(|d| {
            let hpa_replicas = *f!(d.spec, min_replicas).unwrap_or(&0);
            if pdb_min_available >= hpa_replicas {
                let finding = Finding::new(Self::spec(), pdb.metadata.clone())
                    .add_metadata("hpa_replicas", hpa_replicas)
                    .add_metadata("pdb_min_available", pdb_min_available.to_string());
                context.reporter.report(finding);
            }
        })
    }

    fn deploy_matches_with_pdb(pdb: &PodDisruptionBudget, deploy: &Deployment) -> bool {
        f!(pdb.spec, selector) == m!(deploy.spec, selector)
    }
}

#[cfg(test)]
mod tests {
    use crate::linters::lints::pdb_min_replicas::PdbMinReplicas;
    use crate::tests::{analyze_file, filter_findings_by};
    use std::path::Path;

    #[test]
    fn test_pdb_with_deploy_missconfigured() {
        let findings = analyze_file(Path::new("../tests/pdb_deploy_missconfigured.yaml"));
        let findings = filter_findings_by(findings, &PdbMinReplicas::spec());

        assert_eq!(1, findings.len());
        assert_eq!(findings[0].spec().name, "pdb_min_replicas");
        assert_eq!(findings[0].name(), "missconfigured-pdb");
    }

    #[test]
    fn test_pdb_deployment_properly_configured() {
        let findings = analyze_file(Path::new("../tests/pdb_deployment_ok.yaml"));
        let findings = filter_findings_by(findings, &PdbMinReplicas::spec());

        assert_eq!(0, findings.len());
    }

    #[test]
    fn test_pdb_with_hpa_missconfigured() {
        let findings = analyze_file(Path::new("../tests/pdb_hpa_missconfigured.yaml"));
        let findings = filter_findings_by(findings, &PdbMinReplicas::spec());

        assert_eq!(1, findings.len());
        assert_eq!(findings[0].spec().name, "pdb_min_replicas");
        assert_eq!(findings[0].name(), "pdb-hpa-missconfigured");
    }

    #[test]
    fn test_pdb_hpa_ok() {
        let findings = analyze_file(Path::new("../tests/pdb_hpa_ok.yaml"));
        let findings = filter_findings_by(findings, &PdbMinReplicas::spec());

        assert_eq!(0, findings.len());
    }
}
