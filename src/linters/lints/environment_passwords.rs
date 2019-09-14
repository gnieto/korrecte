use crate::linters::{Group, KubeObjectType, Lint, LintSpec};

use crate::reporting::Finding;
use crate::reporting::Reporter;
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
    fn object(&self, object: &KubeObjectType, reporter: &dyn Reporter) {
        let mut visitor = EnvironmentPasswordsVisitor {
            reporter,
            config: &self.config,
        };
        pod_spec_visit(&object, &mut visitor);
    }
}

struct EnvironmentPasswordsVisitor<'a> {
    reporter: &'a dyn Reporter,
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

            self.reporter.report(finding)
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
    use crate::tests::{analyze_file, analyze_file_cfg, filter_findings_by};
    use std::path::Path;

    #[test]
    fn it_finds_passwords_on_pods() {
        let findings = analyze_file(Path::new("tests/secret_on_env_var.yaml"));
        let findings = filter_findings_by(findings, &EnvironmentPasswords::spec());

        assert_eq!(4, findings.len());

        let mut env_vars: Vec<String> = findings
            .into_iter()
            .filter(|f| f.object_metadata().name == "hello-node-hardcoded-env-var")
            .map(|f| f.lint_metadata().get("environment_var").unwrap().clone())
            .collect();
        env_vars.sort();

        assert_eq!(
            env_vars,
            vec![
                "ADMIN_PASSWORD".to_string(),
                "ADMIN_PAssWORD".to_string(),
                "ADMIN_TOKEN".to_string(),
                "KEY_SERVICE".to_string(),
            ]
        );
    }

    #[test]
    fn it_detects_suspicious_with_non_default_config() {
        let config = Config::new(vec!["SUSPICIOUS".to_string(), "ANOTHER".to_string()]);
        let mut global_config = crate::config::Config::default();
        global_config.environment_passwords = config;

        let findings = analyze_file_cfg(Path::new("tests/secret_on_env_var.yaml"), global_config);
        let findings = filter_findings_by(findings, &EnvironmentPasswords::spec());

        assert_eq!(2, findings.len());

        let mut env_vars: Vec<String> = findings
            .into_iter()
            .filter(|f| f.object_metadata().name == "hello-node-hardcoded-env-var")
            .map(|f| f.lint_metadata().get("environment_var").unwrap().clone())
            .collect();
        env_vars.sort();

        assert_eq!(
            env_vars,
            vec!["ENV_ANOTHER".to_string(), "SUSPICIOUS_ENV".to_string(),]
        );
    }
}
