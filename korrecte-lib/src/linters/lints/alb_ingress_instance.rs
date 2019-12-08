use crate::linters::{Group, KubeObjectType, Lint, LintSpec};

use crate::f;
use crate::linters::evaluator::Context;
use crate::reporting::Finding;
use k8s_openapi::api::core::v1::Service;
use k8s_openapi::api::networking::v1beta1::Ingress;
use std::collections::HashSet;

/// **What it does:** Checks that all ALB ingresses are linked to services which have compatible types
/// with the ingress. When the ingress is configured with target-type `instance`, only `NodePort` and `LoadBalancer`
/// types are allowed; when it's configured as `ip`, only `ClusterIP` services are allowed.
///
/// **Why is this bad?** ALB ingress controller will fail to create the associated ALB if services
/// have incompatible types.
///
/// **Known problems:**
///
/// **References**
/// - https://kubernetes-sigs.github.io/aws-alb-ingress-controller/guide/ingress/annotation/#target-type

pub(crate) struct AlbIngressInstance;

impl Lint for AlbIngressInstance {
    fn networking_v1beta1_ingress(&self, ingress: &Ingress, context: &Context) {
        let is_alb_ingress = f!(ingress.metadata, annotations)
            .and_then(|a| a.get("kubernetes.io/ingress.class"))
            .map(|class| class == "alb")
            .unwrap_or(false);

        if !is_alb_ingress {
            return;
        }

        let services = Self::extract_service_names(ingress);
        let ingress_type = IngressType::from(ingress);
        let misconfigured_services =
            self.get_misconfigured_services(context, &ingress_type, services);

        for service in misconfigured_services {
            let finding = Finding::new(Self::spec(), ingress.metadata.clone())
                .add_metadata("service", f!(service.metadata, name).cloned().unwrap());

            context.reporter.report(finding);
        }
    }
}

impl AlbIngressInstance {
    fn spec() -> LintSpec {
        LintSpec {
            group: Group::Configuration,
            name: "alb_ingress_controller_instance_misconfiguration".to_string(),
        }
    }

    fn get_misconfigured_services<'a>(
        &'a self,
        context: &'a Context,
        ingress_type: &IngressType,
        service_names: HashSet<String>,
    ) -> Vec<&'a Service> {
        context
            .repository
            .iter()
            .filter_map(Self::filter_service)
            .filter(|service| {
                Self::is_service_missconfigured(service, ingress_type, &service_names)
            })
            .collect()
    }

    fn filter_service(object: &KubeObjectType) -> Option<&Service> {
        match object {
            KubeObjectType::CoreV1Service(s) => Some(s),
            _ => None,
        }
    }

    fn is_service_missconfigured(
        service: &Service,
        ingress_type: &IngressType,
        service_names: &HashSet<String>,
    ) -> bool {
        let default_service_type = "clusterip".to_string();
        let name = f!(service.metadata, name).unwrap();
        if !service_names.contains(name) {
            return false;
        }

        let service_type = f!(service.spec, type_).unwrap_or(&default_service_type);

        !ingress_type.is_service_type_allowed(&service_type)
    }

    fn extract_service_names(ingress: &Ingress) -> HashSet<String> {
        let empty = Vec::new();

        f!(ingress.spec, rules)
            .unwrap_or(&empty)
            .iter()
            .flat_map(|rule| {
                rule.http.as_ref().map(|http| {
                    http.paths
                        .iter()
                        .map(|path| path.backend.service_name.clone())
                        .collect()
                })
            })
            .collect()
    }
}

enum IngressType {
    Instance,
    Ip,
    Other,
}

impl From<&Ingress> for IngressType {
    fn from(ingress: &Ingress) -> Self {
        let target_type = f!(ingress.metadata, annotations)
            .and_then(|a| a.get("alb.ingress.kubernetes.io/target-type"))
            .map(|target_type| target_type.as_str());

        match target_type {
            Some("instance") => IngressType::Instance,
            Some("ip") => IngressType::Ip,
            None => IngressType::Instance, // By default, Target Type is instance
            _ => IngressType::Other,
        }
    }
}

impl IngressType {
    pub fn is_service_type_allowed(&self, service_type: &str) -> bool {
        let lower_type = service_type.to_lowercase();

        match self {
            IngressType::Instance => lower_type == "nodeport" || lower_type == "loadbalancer",
            IngressType::Ip => lower_type == "clusterip",
            IngressType::Other => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::linters::lints::alb_ingress_instance::AlbIngressInstance;
    use crate::tests::{analyze_file, filter_findings_by};
    use std::path::Path;

    #[test]
    fn it_finds_misconfigured_services_on_alb_ingress_configured_as_instance() {
        let findings = analyze_file(Path::new("../tests/alb_ingress.yaml"));
        let findings = filter_findings_by(findings, &AlbIngressInstance::spec());

        assert_eq!(2, findings.len());
        assert_eq!(findings[0].name(), "missconfigured-alb");
        assert_eq!(
            findings[0].lint_metadata().get("service").unwrap(),
            "service-cluster-ip"
        );

        assert_eq!(findings[1].name(), "missconfigured-alb-ip");
        assert_eq!(
            findings[1].lint_metadata().get("service").unwrap(),
            "service-node-port"
        );
    }
}
