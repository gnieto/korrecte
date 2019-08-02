use crate::linters::{Lint, LintSpec, Group};

use kube::api::Object;
use k8s_openapi::api::core::v1::{PodSpec, PodStatus};
use crate::reporting::{Reporter, Finding};
use k8s_openapi::api::core::v1::{Container, EnvVar};
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
pub(crate) struct EnvironmentPasswords<R: Reporter> {
    config: Config,
    reporter: R,
}

impl<R: Reporter> Lint for EnvironmentPasswords<R> {
    fn spec(&self) -> LintSpec {
        LintSpec {
            group: Group::Security,
            name: "environment_passwords".to_string(),
        }
    }

    fn pod(&self, pod: &Object<PodSpec, PodStatus>) {
        let env_vars_with_secrets: Vec<&EnvVar> = pod.spec.containers
            .iter()
            .map(|c: &Container| c.env.as_ref())
            .flatten()
            .flatten()
            .filter(|envvar| self.is_hardcoded_environment_variable(envvar))
            .collect();


        for environment_var in env_vars_with_secrets {
            let finding = Finding::new(self.spec().clone(), pod.metadata.clone())
                .add_metadata("environment_var".to_string(), environment_var.name.clone());
            self.reporter.report(finding);
        }
    }
}


impl<R: Reporter> EnvironmentPasswords<R> {
    pub fn new(reporter: R, config: Config) -> Self {
        EnvironmentPasswords {
            reporter,
            config,
        }
    }

    fn is_hardcoded_environment_variable(&self, env_var: &EnvVar) -> bool {
        let name = env_var.name.to_uppercase();
        let has_hardcoded_env_var = self.config.suspicious_keys
            .iter()
            .any(|suspicious_key| {
                let key = suspicious_key.to_uppercase();
                name.contains(&key)
            });
        let is_injected = env_var.value.is_none() && env_var.value_from.is_some();

        // If it matches with any of the suspicious substrings and is not injected
        has_hardcoded_env_var && !is_injected
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Config {
    #[serde(default = "default_environment_vars")]
    suspicious_keys: Vec<String>,
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