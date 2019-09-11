use crate::linters::{Lint, LintSpec, Group, KubeObjectType};

use kube::api::Object;
use k8s_openapi::api::apps::v1::{DeploymentSpec, DeploymentStatus};
use k8s_openapi::api::autoscaling::v1::{HorizontalPodAutoscalerSpec, HorizontalPodAutoscalerStatus};
use k8s_openapi::api::policy::v1beta1::{PodDisruptionBudgetSpec, PodDisruptionBudgetStatus};
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use crate::reporting::Finding;
use crate::kube::ObjectRepository;

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
pub(crate) struct PdbMinReplicas<'a> {
    object_repository: &'a Box<dyn ObjectRepository>,
}

impl<'a> Lint for PdbMinReplicas<'a> {
    fn spec(&self) -> LintSpec {
        LintSpec {
            group: Group::Configuration,
            name: "pdb_min_replicas".to_string(),
        }
    }

    fn v1beta1_pod_disruption_budget(&self, pdb: &Object<PodDisruptionBudgetSpec, PodDisruptionBudgetStatus>) -> Vec<Finding> {
        let mut findings = Vec::new();

        if let Some(pdb_min_available) = Self::get_min_replicas(pdb) {
            self.matching_deployments(pdb, &mut findings, pdb_min_available);
            self.matching_hpas(pdb, &mut findings, pdb_min_available);
        }

        findings
    }
}


impl<'a> PdbMinReplicas<'a> {
    pub fn new(object_repository: &'a Box<dyn ObjectRepository>) -> Self {
        PdbMinReplicas {
            object_repository,
        }
    }

    fn get_min_replicas(pdb: &Object<PodDisruptionBudgetSpec, PodDisruptionBudgetStatus>) -> Option<i32> {
        if let Some(IntOrString::Int(amount)) = pdb.spec.max_unavailable {
            return Some(amount);
        }

        None
    }

    fn find_matching_deployments(&self, pdb: &Object<PodDisruptionBudgetSpec, PodDisruptionBudgetStatus>) -> Vec<Object<DeploymentSpec, DeploymentStatus>> {
        self.object_repository.all()
            .iter()
            .filter_map(|o| {
                match o {
                    KubeObjectType::V1Deployment(d) => Some(d),
                    _ => None,
                }
            })
            .filter(|deploy| Self::deploy_matches_with_pdb(pdb, deploy))
            .cloned()
            .collect()
    }

    fn find_matching_hpa(&self, pdb: &Object<PodDisruptionBudgetSpec, PodDisruptionBudgetStatus>) -> Vec<Object<HorizontalPodAutoscalerSpec, HorizontalPodAutoscalerStatus>> {
        self.object_repository.all()
            .iter()
            .filter_map(|o| {
                match o {
                    KubeObjectType::V1HorizontalPodAutoscaler(hpa) => Some(hpa),
                    _ => None,
                }
            })
            .filter(|hpa| {
                if &hpa.spec.scale_target_ref.kind == "Deployment" {
                    // Check if there's any target deployment which is targeted by the PDB
                    self.object_repository
                        .all()
                        .iter()
                        .filter_map(|object| {
                            match object {
                                KubeObjectType::V1Deployment(d) => Some(d),
                                _ => None,
                            }
                        })
                        .any(|deploy| Self::deploy_matches_with_pdb(pdb, &deploy))
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    }

    fn matching_deployments(&self, pdb: &Object<PodDisruptionBudgetSpec, PodDisruptionBudgetStatus>, findings: &mut Vec<Finding>, pdb_min_available: i32) -> () {
        let matching_deployments = self.find_matching_deployments(pdb);

        matching_deployments
            .iter()
            .for_each(|d| {
                let deploy_replicas = d.spec.replicas.unwrap_or(0);

                if pdb_min_available >= deploy_replicas {
                    let finding = Finding::new(self.spec().clone(), pdb.metadata.clone())
                        .add_metadata("deploy_replicas", deploy_replicas)
                        .add_metadata("pdb_min_available", pdb_min_available.to_string());
                    findings.push(finding);
                }
            })
    }

    fn matching_hpas(&self, pdb: &Object<PodDisruptionBudgetSpec, PodDisruptionBudgetStatus>, findings: &mut Vec<Finding>, pdb_min_available: i32) -> () {
        let matching_hpa = self.find_matching_hpa(pdb);

        matching_hpa
            .iter()
            .for_each(|d| {
                let hpa_replicas = d.spec.min_replicas.unwrap_or(0);

                if pdb_min_available >= hpa_replicas {
                    let finding = Finding::new(self.spec().clone(), pdb.metadata.clone())
                        .add_metadata("hpa_replicas", hpa_replicas)
                        .add_metadata("pdb_min_available", pdb_min_available.to_string());
                    findings.push(finding);
                }
            })
    }

    fn deploy_matches_with_pdb(pdb: &Object<PodDisruptionBudgetSpec, PodDisruptionBudgetStatus>, deploy: &Object<DeploymentSpec, DeploymentStatus>) -> bool {
        Some(&deploy.spec.selector) == pdb.spec.selector.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::analyze_file;
    use std::path::Path;

    #[test]
    fn test_pdb_with_deploy_missconfigured() {
        let findings = analyze_file(Path::new("tests/pdb_deploy_missconfigured.yaml"));

        assert_eq!(1, findings.len());
        assert_eq!(findings[0].spec().name, "pdb_min_replicas");
        assert_eq!(findings[0].object_metadata().name, "missconfigured-pdb");
    }

    #[test]
    fn test_pdb_deployment_properly_configured() {
        let findings = analyze_file(Path::new("tests/pdb_deployment_ok.yaml"));

        assert_eq!(0, findings.len());
    }

    #[test]
    fn test_pdb_with_hpa_missconfigured() {
        let findings = analyze_file(Path::new("tests/pdb_hpa_missconfigured.yaml"));

        assert_eq!(1, findings.len());
        assert_eq!(findings[0].spec().name, "pdb_min_replicas");
        assert_eq!(findings[0].object_metadata().name, "pdb-hpa-missconfigured");
    }

    #[test]
    fn test_pdb_hpa_ok() {
        let findings = analyze_file(Path::new("tests/pdb_hpa_ok.yaml"));

        assert_eq!(0, findings.len());
    }
}