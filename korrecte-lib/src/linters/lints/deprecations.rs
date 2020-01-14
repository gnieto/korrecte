use crate::kube::KubeVersion;
use crate::linters::evaluator::Context;
use crate::linters::{Group, Lint, LintSpec};
use crate::reporting::Finding;
use k8s_openapi::api::extensions::v1beta1::Ingress;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use k8s_openapi::Metadata;

/// **What it does:** Checks which Kubernetes objects are deprecated. Note that this rule is applied
/// taking into account the cluster version. For example, a v1beta1/ingress will be considered as
/// deprecated only on a cluster above 1.14, while it won't be considered on older clusters.
///
/// **Why is this bad?** This kind of objects won't be supported on following versions and they
///  should be upgraded as soon as possible.
///
/// **Known problems:**
///
/// **References**
/// - https://github.com/kubernetes/kubernetes/blob/master/CHANGELOG-1.14.md#deprecations
/// - https://kubernetes.io/blog/2019/07/18/api-deprecations-in-1-16/
pub(crate) struct Deprecations;

impl Lint for Deprecations {
    fn extensions_v1beta1_ingress(&self, ingress: &Ingress, context: &Context) {
        Self::check_supported_version(&KubeVersion::new(1, 14), &context, ingress.metadata());
    }
}

impl Deprecations {
    fn spec() -> LintSpec {
        LintSpec {
            group: Group::Configuration,
            name: "deprecations".to_string(),
        }
    }

    fn check_supported_version(
        deprecated_version: &KubeVersion,
        context: &Context,
        metadata: Option<&ObjectMeta>,
    ) {
        let cluster_version = context.repository.version();
        if !Self::is_deprecated(deprecated_version, cluster_version) {
            return;
        }

        let finding = Finding::new(Self::spec(), metadata.cloned());
        context.reporter.report(finding);
    }

    fn is_deprecated(
        deprecated_version: &KubeVersion,
        cluster_version: Option<&KubeVersion>,
    ) -> bool {
        match cluster_version {
            Some(cluster) => cluster >= deprecated_version,
            None => false,
        }
    }
}
