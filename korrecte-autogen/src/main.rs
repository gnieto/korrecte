use inflector::Inflector;
use std::fs::File;
use std::io::Write;
use std::collections::HashSet;

fn main() {
    let specs = [
//        "k8s_openapi::api::core::v1::NamespaceSpec",
        OpenapiResource::new("k8s_openapi::api::core::v1::Node", true),
        OpenapiResource::new("k8s_openapi::api::core::v1::Pod", true),
//        "k8s_openapi::api::core::v1::ReplicationControllerSpec",
        OpenapiResource::new("k8s_openapi::api::core::v1::Service", true),

//        "k8s_openapi::api::apps::v1::DaemonSetSpec",
        OpenapiResource::new("k8s_openapi::api::apps::v1::Deployment", true),
//        "k8s_openapi::api::apps::v1::ReplicaSetSpec",
//        "k8s_openapi::api::apps::v1::StatefulSetSpec",

        OpenapiResource::new("k8s_openapi::api::policy::v1beta1::PodDisruptionBudget", false),

        OpenapiResource::new("k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler", true),
    ];

    let lint = build_lint_trait(&specs);
    let ns = build_imports(&specs);
    let enum_str = build_enum(&specs);

    let source = format!("
{}

{}

{}
", ns, lint, enum_str);

    write_to(&source.trim(), "../src/linters/lint.rs");
    write_to(&build_kube_client(&specs), "../src/kube/api.rs")
}

struct OpenapiResource<'a> {
    resource: &'a str,
    has_kube: bool,
}

impl<'a> OpenapiResource<'a> {
    pub fn new(resource: &'a str, has_kube: bool) -> Self {
        OpenapiResource {
            resource,
            has_kube,
        }
    }

    pub fn variant(&self) -> String {
        let mut split: Vec<&str> = self.resource.split("::").collect();
        split.reverse();

        let object = split.get(0).unwrap();
        let version = split.get(1).unwrap();
        let variant = format!("{}{}", version, object);

        uppercase_first(&variant)
    }

    pub fn clean_name(&self) -> String {
        let mut split: Vec<&str> = self.resource.split("::").collect();
        split.reverse();

        let object = split.get(0).unwrap().to_snake_case();

        object
    }

    pub fn spec(&self) -> String {
        let mut split: Vec<&str> = self.resource.split("::").collect();
        split.reverse();

        let object = split.get(0).unwrap();
        let version = split.get(1).unwrap();
        let ty = split.get(2).unwrap();

        format!("{}::{}::{}Spec", ty, version, object)
    }

    pub fn status(&self) -> String {
        let mut split: Vec<&str> = self.resource.split("::").collect();
        split.reverse();

        let object = split.get(0).unwrap();
        let version = split.get(1).unwrap();
        let ty = split.get(2).unwrap();

        format!("{}::{}::{}Status", ty, version, object)
    }

    pub fn lint_name(&self) -> String {
        let mut split: Vec<&str> = self.resource.split("::").collect();
        split.reverse();

        let version = split.get(1).unwrap();

        format!("{}_{}", version, self.clean_name())
    }

    pub fn api_name(&self) -> Option<String> {
        if !self.has_kube {
            return None;
        }

        let mut split: Vec<&str> = self.resource.split("::").collect();
        split.reverse();

        let object = split.get(0).unwrap();
        let version = split.get(1).unwrap();

        Some(format!("{}{}", version, object))
    }

    pub fn base_namespace(&self) -> String {
        let split: Vec<&str> = self.resource.split("::").collect();
        let count = split.len();

        split[0..count-2].join("::").to_string()
    }

    pub fn parts(&self) -> (&str, &str, &str) {
        let mut split: Vec<&str> = self.resource.split("::").collect();
        split.reverse();

        let object = split.get(0).unwrap();
        let version = split.get(1).unwrap();
        let ty = split.get(2).unwrap();

        (ty, object, version)
    }
}
fn build_kube_client(specs: &[OpenapiResource]) -> String {
    let mut fields = Vec::new();
    let mut inits = Vec::new();
    let mut caches = Vec::new();
    let mut namespaces = HashSet::new();

    for s in specs {
        let maybe_api_name = s.api_name();
        if maybe_api_name.is_none() {
            continue;
        }
        let api_name = maybe_api_name.unwrap();

        fields.push(format!("\t{}: Reflector<Object<{}, {}>>,", s.clean_name(), s.spec(), s.status()));
        inits.push(format!("\t\t\t{}: ApiObjectRepository::initialize_reflector(Api::{}(client.clone()))?,", s.clean_name(), api_name));

        let cache = format!("\t\tobjects.extend(
            api.{}.read()
                    .unwrap()
                    .iter()
                    .map(|o| {{
                        KubeObjectType::{}(Box::new(o.clone()))
                    }})
        );", s.clean_name(), s.variant());
        caches.push(cache);
        namespaces.insert(format!("use {};", s.base_namespace()));
    }

    let namespaces = namespaces.iter().cloned().collect::<Vec<String>>().join("\n");

    format!(
        r#"
use kube::api::{{Object, Reflector, KubeObject, Api}};
use kube::config::Configuration;
use kube::client::APIClient;
use kube::Result;
{}
use serde::de::DeserializeOwned;
use crate::linters::KubeObjectType;
use crate::kube::ObjectRepository;

#[derive(Clone)]
pub struct ApiObjectRepository {{
{}
}}

impl ApiObjectRepository {{
    pub fn new(kube_config: Configuration) -> Result<Self> {{
        let client = APIClient::new(kube_config);

        Ok(ApiObjectRepository {{
{}
        }})
    }}

    fn initialize_reflector<K: 'static + Send + Sync + Clone + DeserializeOwned + KubeObject>(api: Api<K>) -> Result<Reflector<K>> {{
        let mut pod_reflect = Reflector::new(api);
        pod_reflect = pod_reflect.init()?;
        let pod_reflect_updater = pod_reflect.clone();

        std::thread::spawn(move || {{
            loop {{
                if let Err(e) = pod_reflect_updater.poll() {{
                    println!("Error while updating pods: {{}}", e.to_string());
                }}
            }}
        }});

        Ok(pod_reflect)
    }}
}}

pub struct FrozenObjectRepository {{
    objects: Vec<KubeObjectType>,
}}

impl From<ApiObjectRepository> for FrozenObjectRepository {{
    fn from(api: ApiObjectRepository) -> Self {{
        let mut objects = Vec::new();

{}

        FrozenObjectRepository {{
            objects,
        }}
    }}
}}

impl ObjectRepository for FrozenObjectRepository {{
    fn all(&self) -> &Vec<KubeObjectType> {{
        &self.objects
    }}
}}
"#
    , namespaces, fields.join("\n"), inits.join("\n"), caches.join("\n"))
}


fn build_imports(specs: &[OpenapiResource]) -> String {
    let distinct: HashSet<String> = specs.iter()
        .map(|resource|  {
            format!("use {};", resource.base_namespace())
        })
        .collect();

    let mut namespaces = distinct.iter().cloned().collect::<Vec<String>>();
    namespaces.push("use kube::api::Object;".to_string());
    namespaces.push("use crate::linters::LintSpec;".to_string());
    namespaces.push("use crate::error::KorrecteError;".to_string());
    namespaces.join("\n")
}

fn build_lint_trait(specs: &[OpenapiResource]) -> String {
    let mut spec_str = String::new();
    let mut match_arm = Vec::new();

    for s in specs {
        let struct_path = format!("Object<{}, {}>", s.spec(), s.status());

        spec_str.push_str(
            &format!("\tfn {}(&self, _{}: &{}) -> Vec<crate::reporting::Finding> {{ Vec::new() }}\n",
                     s.lint_name(),
                     s.clean_name(),
                     struct_path
            )
        );

        match_arm.push(format!("\t\t\tKubeObjectType::{}(ref o) => self.{}(o),", s.variant(), s.lint_name()));
    }


    format!("pub trait Lint {{
{}
    fn object(&self, object: &KubeObjectType) -> Vec<crate::reporting::Finding> {{
        match object {{
{}
        _ => Vec::new(),
        }}
    }}

    fn spec(&self) -> LintSpec;
}}", spec_str, match_arm.join("\n"))
}

fn build_enum(specs: &[OpenapiResource]) -> String {
    let mut variants = String::new();
    let mut match_arms = Vec::new();

    for s in specs {
        let ty = format!("Object<{}, {}>", s.spec(), s.status());
        variants.push_str(&format!("\t{}(Box<{}>), \n", s.variant(), ty));
        let parts = s.parts();

        let match_arm_str = format!(r##"
            ("{}", "{}", "{}") => {{
				let object = serde_yaml::from_str(yaml)
					.map_err(|_| KorrecteError::FailedToLoadYamlFile)?;

				Ok(KubeObjectType::{}(object))
			}}"##, parts.0, parts.2, parts.1, s.variant()
        );

        match_arms.push(match_arm_str);
    }

    format!("
#[allow(unused)]
pub enum KubeObjectType {{
{}
    #[doc(hidden)]
    __Nonexhaustive,
}}

impl KubeObjectType {{
	pub fn from_yaml(yaml: &str, api_version: &str, kind: &str) -> Result<KubeObjectType, KorrecteError> {{
		let (ty, version) = if api_version.contains('/') {{
			let mut parts = api_version.split('/'N);
			(parts.next().unwrap(), parts.next().unwrap())
		}} else {{
			(\"core\", api_version)
		}};

		match (ty, version, kind) {{
			{}
			_ => Err(KorrecteError::YamlDecodeError {{ty: ty.into(), version: version.into(), kind: kind.into()}}),
		}}
	}}
}}
", variants, match_arms.join("\n"))
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
