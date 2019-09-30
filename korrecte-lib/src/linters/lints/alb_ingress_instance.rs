use crate::linters::{Group, KubeObjectType, Lint, LintSpec};

use crate::kube::ObjectRepository;
use crate::reporting::Finding;
use crate::reporting::Reporter;
use k8s_openapi::api::core::v1::{ServiceSpec, ServiceStatus};
use k8s_openapi::api::networking::v1beta1::{IngressSpec, IngressStatus};
use kube::api::{KubeObject, Object};
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
pub(crate) struct AlbIngressInstance<'a> {
    object_repository: &'a dyn ObjectRepository,
}

impl<'a> AlbIngressInstance<'a> {
    pub fn new(object_repository: &'a dyn ObjectRepository) -> Self {
        AlbIngressInstance { object_repository }
    }
}

impl<'a> Lint for AlbIngressInstance<'a> {
    fn v1beta1_ingress(
        &self,
        ingress: &Object<IngressSpec, IngressStatus>,
        reporter: &dyn Reporter,
    ) {
        let metadata = &ingress.meta().annotations;
        let is_alb_ingress = metadata
            .get("kubernetes.io/ingress.class")
            .map_or(false, |class| class == "alb");
        if !is_alb_ingress {
            return;
        }

        let services = Self::extract_service_names(ingress);
        let ingress_type = IngressType::from(ingress);
        let misconfigured_services = self.get_misconfigured_services(&ingress_type, services);

        for service in misconfigured_services {
            let finding = Finding::new(Self::spec(), ingress.meta().clone())
                .add_metadata("service", service.meta().name.clone());

            reporter.report(finding);
        }
    }
}

impl<'a> AlbIngressInstance<'a> {
    fn spec() -> LintSpec {
        LintSpec {
            group: Group::Configuration,
            name: "alb_ingress_controller_instance_misconfiguration".to_string(),
        }
    }

    fn get_misconfigured_services(
        &self,
        ingress_type: &IngressType,
        service_names: HashSet<String>,
    ) -> Vec<&Object<ServiceSpec, ServiceStatus>> {
        self.object_repository
            .all()
            .iter()
            .filter_map(Self::filter_service)
            .filter(|service| {
                Self::is_service_missconfigured(service, ingress_type, &service_names)
            })
            .collect()
    }

    fn filter_service(object: &KubeObjectType) -> Option<&Object<ServiceSpec, ServiceStatus>> {
        match object {
            KubeObjectType::V1Service(s) => Some(s),
            _ => None,
        }
    }

    fn is_service_missconfigured(
        service: &Object<ServiceSpec, ServiceStatus>,
        ingress_type: &IngressType,
        service_names: &HashSet<String>,
    ) -> bool {
        let default_service_type = "clusterip".to_string();

        if !service_names.contains(&service.meta().name) {
            return false;
        }

        let service_type = service.spec.type_.as_ref().unwrap_or(&default_service_type);
        !ingress_type.is_service_type_allowed(&service_type)
    }

    fn extract_service_names(ingress: &Object<IngressSpec, IngressStatus>) -> HashSet<String> {
        let empty = Vec::new();
        let services: HashSet<String> = ingress
            .spec
            .rules
            .as_ref()
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
            .collect();

        services
    }
}

enum IngressType {
    Instance,
    Ip,
    Other,
}

impl From<&Object<IngressSpec, IngressStatus>> for IngressType {
    fn from(ingress: &Object<IngressSpec, IngressStatus>) -> Self {
        let target_type = ingress
            .meta()
            .annotations
            .get("alb.ingress.kubernetes.io/target-type")
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
        assert_eq!(findings[0].object_metadata().name, "missconfigured-alb");
        assert_eq!(
            findings[0].lint_metadata().get("service").unwrap(),
            "service-cluster-ip"
        );

        assert_eq!(findings[1].object_metadata().name, "missconfigured-alb-ip");
        assert_eq!(
            findings[1].lint_metadata().get("service").unwrap(),
            "service-node-port"
        );
    }
}
