use crate::kube::KubeVersion;
use crate::linters::evaluator::Context;
use crate::linters::{Group, Lint, LintSpec};
use crate::reporting::Finding;
use k8s_openapi::api;
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
    // Deprecations 1.8
    fn extensions_v1beta1_network_policy(
        &self,
        obj: &api::extensions::v1beta1::NetworkPolicy,
        context: &Context,
    ) {
        Self::check_supported_version(&KubeVersion::new(1, 8), &context, obj.metadata());
    }

    // Deprecations 1.9
    fn extensions_v1beta1_daemon_set(
        &self,
        obj: &api::extensions::v1beta1::DaemonSet,
        context: &Context,
    ) {
        Self::check_supported_version(&KubeVersion::new(1, 9), &context, obj.metadata());
    }

    fn apps_v1beta2_daemon_set(
        &self,
        obj: &api::apps::v1beta2::DaemonSet,
        context: &Context,
    ) {
        Self::check_supported_version(&KubeVersion::new(1, 9), &context, obj.metadata());
    }

    fn extensions_v1beta1_deployment(
        &self,
        obj: &api::extensions::v1beta1::Deployment,
        context: &Context,
    ) {
        Self::check_supported_version(&KubeVersion::new(1, 9), &context, obj.metadata());
    }

    fn apps_v1beta2_deployment(
        &self,
        obj: &api::apps::v1beta2::Deployment,
        context: &Context,
    ) {
        // println!("Deployment v1beta2: {:?}", obj);
        Self::check_supported_version(&KubeVersion::new(1, 9), &context, obj.metadata());
    }

    fn extensions_v1beta1_replica_set(
        &self,
        obj: &api::extensions::v1beta1::ReplicaSet,
        context: &Context,
    ) {
        Self::check_supported_version(&KubeVersion::new(1, 9), &context, obj.metadata());
    }

    fn apps_v1beta2_replica_set(
        &self,
        obj: &api::apps::v1beta2::ReplicaSet,
        context: &Context,
    ) {
        Self::check_supported_version(&KubeVersion::new(1, 9), &context, obj.metadata());
    }


    // Deprecations 1.10
    fn extensions_v1beta1_pod_security_policy(
        &self,
        obj: &api::extensions::v1beta1::PodSecurityPolicy,
        context: &Context,
    ) {
        Self::check_supported_version(&KubeVersion::new(1, 10), &context, obj.metadata());
    }

    // Deprecations 1.14
    fn extensions_v1beta1_ingress(
        &self,
        obj: &api::extensions::v1beta1::Ingress,
        context: &Context,
    ) {
        Self::check_supported_version(&KubeVersion::new(1, 14), &context, obj.metadata());
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
