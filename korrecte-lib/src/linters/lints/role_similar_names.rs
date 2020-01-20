use crate::linters::Lint;

use crate::linters::evaluator::Context;
use crate::reporting::Finding;
use k8s_openapi::api::rbac::v1::{ClusterRole, Role};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub(crate) struct RoleSimilarNames;

// We will probably want get this information from swagger/openapi. In the meanwhile, I compiled this with: kubectl api-resources | grep -v NAME | awk '{print $1}' | sed 's/^\|$/"/g'|paste -sd,
const KUBERNETES_OBJECTS: &[&str] = &[
    "bindings",
    "componentstatuses",
    "configmaps",
    "endpoints",
    "events",
    "limitranges",
    "namespaces",
    "nodes",
    "persistentvolumeclaims",
    "persistentvolumes",
    "pods",
    "podtemplates",
    "replicationcontrollers",
    "resourcequotas",
    "secrets",
    "serviceaccounts",
    "services",
    "mutatingwebhookconfigurations",
    "validatingwebhookconfigurations",
    "customresourcedefinitions",
    "apiservices",
    "controllerrevisions",
    "daemonsets",
    "deployments",
    "replicasets",
    "statefulsets",
    "tokenreviews",
    "localsubjectaccessreviews",
    "selfsubjectaccessreviews",
    "selfsubjectrulesreviews",
    "subjectaccessreviews",
    "horizontalpodautoscalers",
    "cronjobs",
    "jobs",
    "certificatesigningrequests",
    "leases",
    "events",
    "daemonsets",
    "deployments",
    "ingresses",
    "networkpolicies",
    "podsecuritypolicies",
    "replicasets",
    "ingresses",
    "networkpolicies",
    "runtimeclasses",
    "poddisruptionbudgets",
    "podsecuritypolicies",
    "clusterrolebindings",
    "clusterroles",
    "rolebindings",
    "roles",
    "priorityclasses",
    "csidrivers",
    "csinodes",
    "storageclasses",
    "volumeattachments",
];

const KUBERNETES_VERBS: &[&str] = &[
    "create",
    "delete",
    "deletecollection",
    "get",
    "list",
    "patch",
    "update",
    "watch",
];

const KUBERNETES_GROUPS: &[&str] = &[
    "",
    "admissionregistration.k8s.io",
    "apiextensions.k8s.io",
    "apiregistration.k8s.io",
    "apps",
    "authentication.k8s.io",
    "authorization.k8s.io",
    "autoscaling",
    "batch",
    "certificates.k8s.io",
    "coordination.k8s.io",
    "events.k8s.io",
    "extensions",
    "networking.k8s.io",
    "node.k8s.io",
    "policy",
    "rbac.authorization.k8s.io",
    "scheduling.k8s.io",
    "storage.k8s.io",
];

const LINT_NAME: &str = "role_similar_names";

impl Lint for RoleSimilarNames {
    fn name(&self) -> &str {
        LINT_NAME
    }

    fn rbac_v1_cluster_role(&self, cluster_role: &ClusterRole, context: &Context) {
        if let Some(ref rules) = cluster_role.rules {
            for rule in rules {
                // Check verbs
                Self::report_similar_names(
                    context,
                    &rule.verbs,
                    KUBERNETES_VERBS,
                    &cluster_role.metadata,
                );

                // Check resources
                if let Some(ref resources) = rule.resources {
                    Self::report_similar_names(
                        context,
                        resources,
                        KUBERNETES_OBJECTS,
                        &cluster_role.metadata,
                    );
                }

                // Check api groups
                if let Some(ref api_groups) = rule.api_groups {
                    Self::report_similar_names(
                        context,
                        api_groups,
                        KUBERNETES_GROUPS,
                        &cluster_role.metadata,
                    );
                }
            }
        }
    }

    fn rbac_v1_role(&self, role: &Role, context: &Context) {
        if let Some(ref rules) = role.rules {
            for rule in rules {
                // Check verbs
                Self::report_similar_names(context, &rule.verbs, KUBERNETES_VERBS, &role.metadata);

                // Check resources
                if let Some(ref resources) = rule.resources {
                    Self::report_similar_names(
                        context,
                        resources,
                        KUBERNETES_OBJECTS,
                        &role.metadata,
                    );
                }

                // Check api groups
                if let Some(ref api_groups) = rule.api_groups {
                    Self::report_similar_names(
                        context,
                        api_groups,
                        KUBERNETES_GROUPS,
                        &role.metadata,
                    );
                }
            }
        }
    }
}

impl RoleSimilarNames {
    fn report_similar_names(
        context: &Context,
        given_names: &[String],
        correct_names: &[&'static str],
        meta: &Option<ObjectMeta>,
    ) {
        let similar_names = RoleSimilarNames::find_similar_names(given_names, correct_names);

        for (incorrect, similar) in similar_names {
            let finding = Finding::new(LINT_NAME, meta.clone())
                .add_metadata("incorrect_name", incorrect)
                .add_metadata("suggested_name", similar.similar_to);
            context.reporter.report(finding);
        }
    }

    fn find_similar_names<'a>(
        given_names: &'a [String],
        correct_names: &[&'static str],
    ) -> HashMap<&'a str, SimilarName> {
        let mut similar_names = HashMap::new();

        for current_name in given_names {
            if correct_names.contains(&current_name.as_str()) {
                // If the current name is contained on the list of valid names, do not perform any additional check
                continue;
            }

            if current_name == "*" {
                // Avoid also matching on wildcards
                continue;
            }

            for correct_name in correct_names.iter() {
                let distance = levenshtein::levenshtein(current_name, correct_name);
                Self::store_similar_name(&mut similar_names, correct_name, current_name, distance);
            }
        }

        similar_names
    }

    fn store_similar_name<'a>(
        similar_names: &mut HashMap<&'a str, SimilarName>,
        correct_resource: &'static str,
        current_resource: &'a str,
        distance: usize,
    ) {
        if distance == 1 || distance == 2 {
            match similar_names.entry(current_resource) {
                Entry::Vacant(entry) => {
                    let similar = SimilarName {
                        similar_to: correct_resource,
                        distance,
                    };
                    entry.insert(similar);
                }
                Entry::Occupied(mut entry) => {
                    if distance < entry.get().distance {
                        let similar = SimilarName {
                            similar_to: correct_resource,
                            distance,
                        };
                        entry.insert(similar);
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct SimilarName {
    similar_to: &'static str,
    distance: usize,
}

#[cfg(test)]
mod tests {
    use crate::reporting::Finding;
    use crate::tests::{analyze_file, filter_findings_by};
    use std::path::Path;

    #[test]
    fn it_finds_similar_object_names_on_cluster_role() {
        let findings = analyze_file(Path::new("../tests/role_similar_names_cluster_role.yaml"));
        let findings = filter_findings_by(findings, super::LINT_NAME);

        assert!(has_suggestion(
            &findings,
            "korrecte-cluster-role",
            "nodas",
            "nodes"
        ));
        assert!(has_suggestion(
            &findings,
            "korrecte-cluster-role",
            "noeds",
            "nodes"
        ));
        assert!(has_suggestion(
            &findings,
            "korrecte-cluster-role",
            "pids",
            "pods"
        ));
        assert!(has_suggestion(
            &findings,
            "korrecte-cluster-role",
            "daemonset",
            "daemonsets"
        ));
        assert!(has_suggestion(
            &findings,
            "korrecte-cluster-role",
            "wstch",
            "watch"
        ));
        assert!(has_suggestion(
            &findings,
            "korrecte-cluster-role",
            "appps",
            "apps"
        ));
        assert_eq!(6, findings.len());
    }

    #[test]
    fn it_finds_similar_object_names_on_role() {
        let findings = analyze_file(Path::new("../tests/role_similar_names_role.yaml"));
        let findings = filter_findings_by(findings, super::LINT_NAME);

        assert!(has_suggestion(&findings, "korrecte-role", "a", ""));
        assert!(has_suggestion(
            &findings,
            "korrecte-role",
            "ingressess",
            "ingresses"
        ));
        assert!(has_suggestion(&findings, "korrecte-role", "lust", "list"));
        assert_eq!(3, findings.len());
    }

    fn has_suggestion(
        findings: &Vec<Finding>,
        name: &str,
        incorrect_name: &str,
        suggested_name: &str,
    ) -> bool {
        return findings.iter().any(|f| {
            f.name() == name
                && f.lint_metadata().get("incorrect_name").unwrap() == incorrect_name
                && f.lint_metadata().get("suggested_name").unwrap() == suggested_name
        });
    }
}
