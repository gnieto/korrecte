use crate::linters::{Lint, LintSpec, Group};
use crate::linters::evaluator::Context;
use k8s_openapi::api::extensions::v1beta1::Ingress;
use crate::reporting::Finding;
use k8s_openapi::Metadata;

pub(crate) struct Deprecations;

impl Lint for Deprecations {
    fn extensions_v1beta1_ingress(&self, ingress: &Ingress, context: &Context) {
        let finding = Finding::new(
            Self::spec(),
            ingress.metadata().cloned()
        );

        context.reporter.report(finding);
    }
}

impl Deprecations {
    fn spec() -> LintSpec {
        LintSpec {
            group: Group::Configuration,
            name: "deprecations".to_string(),
        }
    }
}