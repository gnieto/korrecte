use inflector::Inflector;
use std::fs::File;
use std::io::Write;

fn main() {
    let specs = [
        //        "k8s_openapi::api::core::v1::NamespaceSpec",
        OpenapiResource::new("k8s_openapi::api::core::v1::Node"),
        OpenapiResource::new("k8s_openapi::api::core::v1::Pod"),
        //        "k8s_openapi::api::core::v1::ReplicationControllerSpec",
        OpenapiResource::new("k8s_openapi::api::core::v1::Service"),
        OpenapiResource::new("k8s_openapi::api::apps::v1::DaemonSet"),
        OpenapiResource::new("k8s_openapi::api::apps::v1::Deployment"),
        OpenapiResource::new("k8s_openapi::api::apps::v1::ReplicaSet"),
        OpenapiResource::new("k8s_openapi::api::apps::v1::StatefulSet"),
        OpenapiResource::new("k8s_openapi::api::policy::v1beta1::PodDisruptionBudget"),
        OpenapiResource::new("k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler"),
        OpenapiResource::new("k8s_openapi::api::autoscaling::v2beta1::HorizontalPodAutoscaler"),
        OpenapiResource::new("k8s_openapi::api::autoscaling::v2beta2::HorizontalPodAutoscaler"),
        OpenapiResource::new("k8s_openapi::api::networking::v1beta1::Ingress"),
        OpenapiResource::new("k8s_openapi::api::extensions::v1beta1::Ingress"),
        OpenapiResource::new("k8s_openapi::api::rbac::v1::ClusterRole"),
        OpenapiResource::new("k8s_openapi::api::rbac::v1::Role"),
    ];

    let lint = build_lint_trait(&specs);
    let ns = build_imports();
    let enum_str = build_enum(&specs);
    let from_impl = build_from_impls(&specs);

    let source = format!(
        "
{}

{}

{}

{}
",
        ns, lint, enum_str, from_impl
    );

    write_to(&source.trim(), "../korrecte-lib/src/linters/lint.rs");
    write_to(
        &build_kube_client(&specs),
        "../korrecte-lib/src/kube/api_async.rs",
    )
}

struct OpenapiResource<'a> {
    resource: &'a str,
}

impl<'a> OpenapiResource<'a> {
    pub fn new(resource: &'a str) -> Self {
        OpenapiResource { resource }
    }

    pub fn variant(&self) -> String {
        let mut split: Vec<&str> = self.resource.split("::").collect();
        split.reverse();

        let object = split.get(0).unwrap();
        let version = uppercase_first(split.get(1).unwrap());
        let group = split.get(2).unwrap();
        let variant = format!("{}{}{}", group, version, object);

        uppercase_first(&variant)
    }

    pub fn clean_name(&self) -> String {
        let mut split: Vec<&str> = self.resource.split("::").collect();
        split.reverse();

        split.get(0).unwrap().to_snake_case()
    }

    pub fn fqn(&self) -> &str {
        self.resource
    }

    pub fn lint_name(&self) -> String {
        let mut split: Vec<&str> = self.resource.split("::").collect();
        split.reverse();

        let version = split.get(1).unwrap();
        let group = lowercase_first(split.get(2).unwrap());

        format!("{}_{}_{}", group, version, self.clean_name())
    }

    pub fn parts(&self) -> (&str, &str, &str) {
        let mut split: Vec<&str> = self.resource.split("::").collect();
        split.reverse();

        let object = split.get(0).unwrap();
        let version = split.get(1).unwrap();
        let mut ty = split.get(2).unwrap();

        if ty == &"networking" {
            ty = &"networking.k8s.io";
        }

        if ty == &"rbac" {
            ty = &"rbac.authorization.k8s.io";
        }

        (ty, object, version)
    }
}

fn build_async_requests(resource: &OpenapiResource) -> String {
    return format!(
        r#"v.push(self.reflector_for::<{fqn}>("{res}").boxed());"#,
        fqn = resource.fqn(),
        res = resource.variant()
    );
}

fn build_kube_client(specs: &[OpenapiResource]) -> String {
    let mut reflectors = Vec::new();

    for s in specs {
        reflectors.push(build_async_requests(s));
    }

    format!(
        r#"use crate::kube::repository::ObjectRepository;
use crate::linters::KubeObjectType;
use anyhow::*;
use futures::future::Future;
use std::pin::Pin;
use kube::api::{{Resource, ListParams, Meta}};
use kube::runtime::Reflector;
use futures::FutureExt;
use serde::de::DeserializeOwned;

pub struct ApiObjectRepository {{
    kubeclient: kube::client::APIClient,
}}

type ReflectorFuture<'a> = Box<dyn Future<Output=Result<Vec<KubeObjectType>, anyhow::Error>> + 'a>;

impl ApiObjectRepository {{
    pub fn new() -> Result<Self> {{
        let config = futures::executor::block_on(kube::config::load_kube_config())?;
        let kubeclient = kube::client::APIClient::new(config);
        Ok(Self {{ kubeclient }})
    }}

    pub async fn load_all_objects(&self) -> Result<Vec<KubeObjectType>, ()> {{
        let mut v: Vec<Pin<ReflectorFuture>> = Vec::new();
        let mut objects = Vec::new();

        {}

        let all_futures: Vec<Result<Vec<KubeObjectType>, anyhow::Error>> =
            futures::future::join_all(v).await;

        for f in all_futures {{
            match f {{
                Err(ref e) => println!("Error loading some resource: {{}}", e),
                Ok(current) => objects.extend(current),
            }}
        }}

        Ok(objects)
    }}

    pub async fn reflector_for<R: ReflectorFor>(&self, ty: &'static str) -> Result<Vec<KubeObjectType>, anyhow::Error> {{
        let client = self.kubeclient.clone();

        let reflector = Reflector::<R>::new(
            client,
            ListParams::default(),
            Resource::all::<R>()
        );
        let reflector = reflector.init().await?;

        reflector
            .state()
            .await
            .map(|objects| {{
                objects.iter()
                    .map(|obj| obj.clone().into())
                    .collect()
            }})
            .map_err(|e| anyhow!("Err loading {{}}: {{}}", ty, e))
    }}
}}

pub trait ReflectorFor: Clone+Send+Meta+DeserializeOwned+Into<KubeObjectType> {{}}
impl<T: Clone+Send+Meta+DeserializeOwned+Into<KubeObjectType>> ReflectorFor for T {{}}

pub struct FrozenObjectRepository {{
    objects: Vec<KubeObjectType>,
}}

impl From<ApiObjectRepository> for FrozenObjectRepository {{
    fn from(api: ApiObjectRepository) -> Self {{
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let all_objects = rt.block_on(api.load_all_objects()).unwrap();

        FrozenObjectRepository {{
            objects: all_objects,
        }}
    }}
}}

impl ObjectRepository for FrozenObjectRepository {{
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a KubeObjectType> + 'a> {{
        Box::new(self.objects.iter())
    }}
}}
"#,
        reflectors.join("\n")
    )
}

fn build_imports() -> String {
    let mut namespaces = Vec::new();
    namespaces.push("use crate::linters::evaluator::Context;".to_string());
    namespaces.push("use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;".to_string());
    namespaces.push("use anyhow::{Result, anyhow};".to_string());
    namespaces.join("\n")
}

fn build_lint_trait(specs: &[OpenapiResource]) -> String {
    let mut spec_str = String::new();
    let mut match_arm = Vec::new();

    for s in specs {
        let struct_path = s.fqn();

        spec_str.push_str(&format!(
            "\tfn {}(&self, _{}: &{}, _context: &Context)  {{  }}\n",
            s.lint_name(),
            s.clean_name(),
            struct_path
        ));

        match_arm.push(format!(
            "\t\t\tKubeObjectType::{}(ref o) => self.{}(o, context),",
            s.variant(),
            s.lint_name()
        ));
    }

    format!(
        "pub trait Lint {{
    fn name(&self) -> &str;
{}
    fn object(&self, object: &KubeObjectType, context: &Context) {{
        match object {{
{}
        }}
    }}
}}",
        spec_str,
        match_arm.join("\n")
    )
}

fn build_from_impls(specs: &[OpenapiResource]) -> String {
    let mut froms = Vec::new();

    for s in specs {
        let template = format!(
            r##"
impl From<{}> for KubeObjectType {{
    fn from(o: {}) -> Self {{
        Self::{}(Box::new(o))
    }}
}}"##,
            s.fqn(),
            s.fqn(),
            s.variant()
        );

        froms.push(template)
    }

    froms.join("\n")
}

fn build_enum(specs: &[OpenapiResource]) -> String {
    let mut variants = String::new();
    let mut match_arms = Vec::new();
    let mut types_arms = Vec::new();
    let mut metadata_arms = Vec::new();

    for s in specs {
        let ty = s.fqn();
        variants.push_str(&format!("\t{}(Box<{}>), \n", s.variant(), ty));
        let parts = s.parts();

        let match_arm_str = format!(
            r##"
            ("{}", "{}", "{}") => {{
				let object = serde_yaml::from_str(yaml)?;

				Ok(KubeObjectType::{}(object))
			}}"##,
            parts.0,
            parts.2,
            parts.1,
            s.variant()
        );
        match_arms.push(match_arm_str);

        let types_arm_str = format!(
            r##"
            KubeObjectType::{}(_) => {{
                ty == "{}" && version == "{}" && kind == "{}"
            }}
            "##,
            s.variant(),
            parts.0,
            parts.2,
            parts.1,
        );
        types_arms.push(types_arm_str);

        let metadata_arm_str = format!(
            r##"
            KubeObjectType::{}(ref o) => o.metadata.as_ref(),
            "##,
            s.variant(),
        );
        metadata_arms.push(metadata_arm_str);
    }

    format!("
#[allow(unused)]
pub enum KubeObjectType {{
{}
}}

impl KubeObjectType {{
	pub fn from_yaml(yaml: &str, api_version: &str, kind: &str) -> Result<KubeObjectType, anyhow::Error> {{
		let (ty, version) = if api_version.contains('/') {{
			let mut parts = api_version.split('/');
			(parts.next().unwrap(), parts.next().unwrap())
		}} else {{
			(\"core\", api_version)
		}};

		match (ty, version, kind) {{
			{}
			_ => Err(anyhow!(\"Could not decode the given object type\"))
		}}
	}}

    pub fn matches_type(&self, api_version: &str, kind: &str) -> bool {{
        let (ty, version) = if api_version.contains('/') {{
			let mut parts = api_version.split('/');
			(parts.next().unwrap(), parts.next().unwrap())
		}} else {{
			(\"core\", api_version)
		}};

        match *self {{
            {}
        }}
    }}

    pub fn metadata(&self) -> Option<&ObjectMeta> {{
        match *self {{
            {}
        }}
    }}
}}
", variants, match_arms.join("\n"), types_arms.join("\n"), metadata_arms.join("\n"))
}

fn write_to(content: &str, path: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn uppercase_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[allow(dead_code)]
fn lowercase_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    }
}
