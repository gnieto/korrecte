use crate::linters::{Group, KubeObjectType, Lint, LintSpec};

use crate::reporting::Finding;
use crate::visitor::{pod_spec_visit, PodSpecVisitor};
use k8s_openapi::api::core::v1::PodSpec;
use k8s_openapi::api::core::v1::{Container, EnvVar};
use kube::api::ObjectMeta;
use serde::Deserialize;

/// **What it does:** Finds passwords or keys on object manifests.
///
/// **Why is this bad?** This passwords or keys are visible to anyone with access to this manifests.
/// You can use `secret` objects and inject them through enviorment variables.
///
/// **Known problems:**
///
/// **References:**
/// - https://kubernetes.io/docs/concepts/configuration/secret/
/// - https://kubernetes.io/docs/tasks/inject-data-application/distribute-credentials-secure/
pub(crate) struct EnvironmentPasswords {
    config: Config,
}

impl Lint for EnvironmentPasswords {
    fn object(&self, object: &KubeObjectType) -> Vec<Finding> {
        let mut visitor = EnvironmentPasswordsVisitor {
            findings: Vec::new(),
            config: &self.config,
        };
        pod_spec_visit(&object, &mut visitor);

        visitor.findings
    }
}

struct EnvironmentPasswordsVisitor<'a> {
    findings: Vec<Finding>,
    config: &'a Config,
}

impl PodSpecVisitor for EnvironmentPasswordsVisitor<'_> {
    fn visit_pod_spec(&mut self, pod_spec: &PodSpec, meta: &ObjectMeta) {
        let env_vars_with_secrets: Vec<&EnvVar> = pod_spec
            .containers
            .iter()
            .map(|c: &Container| c.env.as_ref())
            .flatten()
            .flatten()
            .filter(|envvar| self.is_hardcoded_environment_variable(envvar))
            .collect();

        for environment_var in env_vars_with_secrets {
            let finding = Finding::new(EnvironmentPasswords::spec(), meta.clone())
                .add_metadata("environment_var".to_string(), environment_var.name.clone());

            self.findings.push(finding);
        }
    }
}

impl EnvironmentPasswordsVisitor<'_> {
    fn is_hardcoded_environment_variable(&self, env_var: &EnvVar) -> bool {
        let name = env_var.name.to_uppercase();
        let has_hardcoded_env_var = self.config.suspicious_keys.iter().any(|suspicious_key| {
            let key = suspicious_key.to_uppercase();
            name.contains(&key)
        });
        let is_injected = env_var.value.is_none() && env_var.value_from.is_some();

        // If it matches with any of the suspicious substrings and is not injected
        has_hardcoded_env_var && !is_injected
    }
}

impl EnvironmentPasswords {
    pub fn new(config: Config) -> Self {
        EnvironmentPasswords { config }
    }

    fn spec() -> LintSpec {
        LintSpec {
            group: Group::Security,
            name: "environment_passwords".to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Config {
    #[serde(default = "default_environment_vars")]
    suspicious_keys: Vec<String>,
}

impl Config {
    #[allow(unused)]
    pub fn new(suspicious_keys: Vec<String>) -> Self {
        Config { suspicious_keys }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            suspicious_keys: default_environment_vars(),
        }
    }
}

fn default_environment_vars() -> Vec<String> {
    vec![
        "password".to_string(),
        "token".to_string(),
        "key".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use crate::linters::lints::environment_passwords::{Config, EnvironmentPasswords};
    use crate::linters::{KubeObjectType, Lint};
    use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
    use kube::api::Object;
    use serde_json::json;
    use serde_json::Value;

    #[test]
    fn it_finds_passwords_on_pods() {
        let envvars = json!([
            {
                "name": "ADMIN_PASSWORD",
                "value": "changeme"
            },
            {
                "name": "ADMIN_NON_SUSPICIOUS_WORD",
                "value": "no-password"
            }
        ]);
        let pod = get_pod_with_environment_vars(envvars);
        let linter = EnvironmentPasswords::new(Config::default());

        let findings = linter.object(&KubeObjectType::V1Pod(Box::new(pod)));
        assert_eq!(1, findings.len());
        let finding = &findings[0];
        assert_eq!(finding.spec(), &EnvironmentPasswords::spec());
        assert_eq!(
            "ADMIN_PASSWORD",
            finding
                .lint_metadata()
                .get("environment_var".into())
                .unwrap()
        );
    }

    #[test]
    fn it_does_not_detect_if_source_is_not_literal() {
        let envvars = json!([
            {
                "name": "ADMIN_PASSWORD",
                "valueFrom": {
                    "secretKeyFrom": {
                        "name": "some-secret",
                        "value": "secret-key",
                    }
                }
            }
        ]);
        let pod = get_pod_with_environment_vars(envvars);

        let linter = EnvironmentPasswords::new(Config::default());

        let lints = linter.v1_pod(&pod);
        assert_eq!(0, lints.len());
    }

    #[test]
    fn it_detects_suspicious_keys_on_mixed_cases() {
        let envvars = json!([
            {
                "name": "ADMIN_PAssWORD",
                "value": "changeme"
            }
        ]);
        let pod = get_pod_with_environment_vars(envvars);

        let linter = EnvironmentPasswords::new(Config::default());

        let findings = linter.object(&KubeObjectType::V1Pod(Box::new(pod)));
        assert_eq!(1, findings.len());
        let finding = &findings[0];
        assert_eq!(finding.spec(), &EnvironmentPasswords::spec());
        assert_eq!(
            "ADMIN_PAssWORD",
            finding
                .lint_metadata()
                .get("environment_var".into())
                .unwrap()
        );
    }

    #[test]
    fn it_detects_suspicious_with_non_default_config() {
        let envvars = json!([
            {
                "name": "ADMIN_PASSWORD",
                "value": "changeme"
            },
            {
                "name": "SUSPICIOUS_KEY",
                "value": "changeme"
            },
            {
                "name": "ENV_ANOTHER_KEY",
                "value": "randomvalue",
            }
        ]);
        let pod = get_pod_with_environment_vars(envvars);

        let config = Config::new(vec!["SUSPICIOUS".to_string(), "ANOTHER_KEY".to_string()]);
        let linter = EnvironmentPasswords::new(config);

        let findings = linter.object(&KubeObjectType::V1Pod(Box::new(pod)));

        assert_eq!(2, findings.len());
        let finding = &findings[0];
        assert_eq!(finding.spec(), &EnvironmentPasswords::spec());
        assert_eq!(
            "SUSPICIOUS_KEY",
            finding
                .lint_metadata()
                .get("environment_var".into())
                .unwrap()
        );

        let finding = &findings[1];
        assert_eq!(finding.spec(), &EnvironmentPasswords::spec());
        assert_eq!(
            "ENV_ANOTHER_KEY",
            finding
                .lint_metadata()
                .get("environment_var".into())
                .unwrap()
        );
    }

    fn get_pod_with_environment_vars(json_value: Value) -> Object<PodSpec, PodStatus> {
        let pod_spec = json!({
            "apiVersion": "v1",
            "kind": "Pod",
            "metadata":  {
                "name": "pod-name",
                "namespaces": "test-ns",
            },
            "spec": {
                "containers": [{
                    "name": "app",
                    "image": "some-image",
                    "env": json_value,
                }]
            }
        });
        let pod: Object<PodSpec, PodStatus> = serde_json::from_value(pod_spec).unwrap();

        pod
    }
}
