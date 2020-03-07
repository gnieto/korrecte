use crate::f;
use crate::linters::evaluator::Context;
use crate::linters::Lint;
use crate::reporting::Finding;
use k8s_openapi::api::networking::v1beta1::Ingress;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

pub(crate) struct AlbNamedSecurityGroups;

const LINT_NAME: &str = "alb_named_sg";
const ANNOTATION_NAME: &str = "alb.ingress.kubernetes.io/security-groups";

impl Lint for AlbNamedSecurityGroups {
    fn name(&self) -> &str {
        LINT_NAME
    }

    fn extensions_v1beta1_ingress(
        &self,
        ingress: &k8s_openapi::api::extensions::v1beta1::Ingress,
        context: &Context,
    ) {
        let security_groups =
            f!(ingress.metadata, annotations).and_then(|a| a.get(ANNOTATION_NAME));

        if let Some(security_groups) = security_groups {
            Self::check_security_groups(security_groups, &ingress.metadata, context);
        }
    }

    fn networking_v1beta1_ingress(&self, ingress: &Ingress, context: &Context) {
        let security_groups =
            f!(ingress.metadata, annotations).and_then(|a| a.get(ANNOTATION_NAME));

        if let Some(security_groups) = security_groups {
            Self::check_security_groups(security_groups, &ingress.metadata, context);
        }
    }
}

impl AlbNamedSecurityGroups {
    fn check_security_groups(
        security_groups: &str,
        metadata: &Option<ObjectMeta>,
        context: &Context,
    ) {
        let uses_security_group_id: Vec<&str> = security_groups
            .split(',')
            .filter(|security_group| security_group.starts_with("sg-"))
            .collect();

        if uses_security_group_id.is_empty() {
            return;
        }

        let finding = Finding::new(LINT_NAME, metadata.clone())
            .add_metadata("invalid_security_groups", uses_security_group_id.join(","));
        context.reporter.report(finding);
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{analyze_file, filter_findings_by};
    use std::path::Path;

    #[test]
    fn checks_alb_named_security_groups() {
        let findings = analyze_file(Path::new("../tests/alb_named_sg.yaml"));
        let findings = filter_findings_by(findings, super::LINT_NAME);

        assert_eq!(1, findings.len());
        assert_eq!("alb-named-incorrect", findings[0].name());
    }
}
