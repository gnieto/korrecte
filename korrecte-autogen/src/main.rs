use inflector::Inflector;
use std::fs::File;
use std::io::Write;
use std::collections::HashSet;

fn main() {
    let specs = [
//        "k8s_openapi::api::core::v1::NamespaceSpec",
        "k8s_openapi::api::core::v1::NodeSpec",
        "k8s_openapi::api::core::v1::PodSpec",
//        "k8s_openapi::api::core::v1::ReplicationControllerSpec",
        "k8s_openapi::api::core::v1::ServiceSpec",

//        "k8s_openapi::api::apps::v1::DaemonSetSpec",
        "k8s_openapi::api::apps::v1::DeploymentSpec",
//        "k8s_openapi::api::apps::v1::ReplicaSetSpec",
//        "k8s_openapi::api::apps::v1::StatefulSetSpec",

        // "k8s_openapi::api::policy::v1beta1::PodDisruptionBudgetSpec",
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

fn build_kube_client(specs: &[&str]) -> String {

    let namespaces = r#"
use k8s_openapi::api::core;
use k8s_openapi::api::apps;

"#;

    let mut fields = Vec::new();
    let mut inits = Vec::new();
    let mut caches = Vec::new();

    for s in specs {
        let mut split: Vec<&str> = s.split("::").collect();
        split.reverse();

        let object = split.get(0).unwrap();

        let clean_name = split.get(0).unwrap()
            .trim_end_matches("Spec")
            .to_snake_case();
        let version = split.get(1).unwrap();
        let ty = split.get(2).unwrap();

        let spec = format!("{}::{}::{}", ty, version, object);
        let status = spec.replace("Spec", "Status");
        let api_name = format!("{}{}", version, object.replace("Spec", ""));
        let variant = format!("{}{}", version, object.replace("Spec", ""));


        fields.push(format!("\t{}: Reflector<Object<{}, {}>>,", clean_name, spec, status));
        inits.push(format!("\t\t\t{}: ApiObjectRepository::initialize_reflector(Api::{}(client.clone()))?,", clean_name, api_name));

        let cache = format!("\t\tobjects.extend(
            api.{}.read()
                    .unwrap()
                    .iter()
                    .map(|o| {{
                        KubeObjectType::{}(o.clone())
                    }})
        );", clean_name, uppercase_first(&variant));
        caches.push(cache);
    }
    let assignements = "";

    format!(
        r#"
use kube::api::{{Object, Reflector, KubeObject, Api}};
use kube::config::Configuration;
use kube::client::APIClient;
use kube::Result;
{}
use serde::de::DeserializeOwned;
use super::Identifier;
use crate::linters::KubeObjectType;
use crate::kube::NewObjectRepository;

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

impl NewObjectRepository for FrozenObjectRepository {{
    fn all(&self) -> &Vec<KubeObjectType> {{
        &self.objects
    }}

    fn find(&self, _id: &Identifier) -> Option<&KubeObjectType> {{
        unimplemented!()
    }}
}}
"#
    , namespaces, fields.join("\n"), inits.join("\n"), caches.join("\n"))
}


fn build_imports(specs: &[&str]) -> String {
    let distinct: HashSet<String> = specs.iter()
        .map(|namespace|  {
            let split: Vec<&str> = namespace.split("::").collect();
            let count = split.len();
            let ns = split[0..count-2].join("::").to_string();

            format!("use {};", ns)
        })
        .collect();

    let mut namespaces = distinct.iter().cloned().collect::<Vec<String>>();
    namespaces.push("use kube::api::Object;".to_string());
    namespaces.push("use crate::linters::LintSpec;".to_string());
    namespaces.join("\n")
}

fn build_lint_trait(specs: &[&str]) -> String {
    let mut spec_str = String::new();

    for s in specs {
        let mut split: Vec<&str> = s.split("::").collect();
        split.reverse();

        let object = split.get(0).unwrap();

        let clean_name = split.get(0).unwrap()
            .trim_end_matches("Spec")
            .to_snake_case();
        let version = split.get(1).unwrap();
        let ty = split.get(2).unwrap();
        let method_name = format!("{}_{}", version, clean_name);

        let spec = object;
        let status = object.replace("Spec", "Status");
        let struct_path = format!("Object<{}::{}::{}, {}::{}::{}>", ty, version, spec, ty, version, status);

        spec_str.push_str(&format!("\tfn {}(&self, _{}: &{}) -> Vec<crate::reporting::Finding> {{ Vec::new() }}\n", method_name, clean_name, struct_path));
    }


    format!("pub trait Lint {{
{}
    fn spec(&self) -> LintSpec;
}}", spec_str)
}

fn build_enum(specs: &[&str]) -> String {
    let mut variants = String::new();

    for s in specs {
        let mut split: Vec<&str> = s.split("::").collect();
        split.reverse();

        let object = split.get(0).unwrap();

        let clean_name = split.get(0).unwrap()
            .trim_end_matches("Spec");
        let version = split.get(1).unwrap();
        let ty = split.get(2).unwrap();
        let variant = format!("{}{}", version, clean_name);

        let spec = object;
        let status = object.replace("Spec", "Status");
        let ty = format!("Object<{}::{}::{}, {}::{}::{}>", ty, version, spec, ty, version, status);

        variants.push_str(&format!("\t{}({}), \n", uppercase_first(&variant), ty));
    }


    format!("pub enum KubeObjectType {{
{}
    #[doc(hidden)]
    __Nonexhaustive,
}}", variants)
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
