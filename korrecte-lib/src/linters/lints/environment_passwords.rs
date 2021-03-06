use crate::linters::{KubeObjectType, Lint};

use crate::linters::evaluator::Context;
use crate::reporting::Finding;
use crate::visitor::{pod_spec_visit, PodSpecVisitor};
use k8s_openapi::api::core::v1::PodSpec;
use k8s_openapi::api::core::v1::{Container, EnvVar};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use serde::Deserialize;

pub(crate) struct EnvironmentPasswords {
    config: Config,
}

const LINT_NAME: &str = "environment_passwords";

impl Lint for EnvironmentPasswords {
    fn name(&self) -> &str {
        LINT_NAME
    }

    fn object(&self, object: &KubeObjectType, context: &Context) {
        let mut visitor = EnvironmentPasswordsVisitor {
            context: &context,
            config: &self.config,
        };
        pod_spec_visit(&object, &mut visitor);
    }
}

struct EnvironmentPasswordsVisitor<'a> {
    context: &'a Context<'a>,
    config: &'a Config,
}

impl PodSpecVisitor for EnvironmentPasswordsVisitor<'_> {
    fn visit_pod_spec(&mut self, pod_spec: &PodSpec, _: &ObjectMeta, meta: Option<&ObjectMeta>) {
        let env_vars_with_secrets: Vec<&EnvVar> = pod_spec
            .containers
            .iter()
            .map(|c: &Container| c.env.as_ref())
            .flatten()
            .flatten()
            .filter(|envvar| self.is_hardcoded_environment_variable(envvar))
            .collect();

        for environment_var in env_vars_with_secrets {
            let finding = Finding::new(LINT_NAME, meta.cloned())
                .add_metadata("environment_var".to_string(), environment_var.name.clone());

            self.context.reporter.report(finding)
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
    use crate::linters::lints::environment_passwords::Config;
    use crate::tests::{analyze_file, analyze_file_cfg, filter_findings_by};
    use std::path::Path;

    #[test]
    fn it_finds_passwords_on_pods() {
        let findings = analyze_file(Path::new("../tests/secret_on_env_var.yaml"));
        let findings = filter_findings_by(findings, super::LINT_NAME);

        assert_eq!(4, findings.len());

        let mut env_vars: Vec<String> = findings
            .into_iter()
            .filter(|f| f.name() == "hello-node-hardcoded-env-var")
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

        let findings =
            analyze_file_cfg(Path::new("../tests/secret_on_env_var.yaml"), global_config);
        let findings = filter_findings_by(findings, super::LINT_NAME);

        assert_eq!(2, findings.len());

        let mut env_vars: Vec<String> = findings
            .into_iter()
            .filter(|f| f.name() == "hello-node-hardcoded-env-var")
            .map(|f| f.lint_metadata().get("environment_var").unwrap().clone())
            .collect();
        env_vars.sort();

        assert_eq!(
            env_vars,
            vec!["ENV_ANOTHER".to_string(), "SUSPICIOUS_ENV".to_string(),]
        );
    }
}
