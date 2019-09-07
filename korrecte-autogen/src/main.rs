use inflector::Inflector;
use std::fs::File;
use std::io::Write;
use std::collections::HashSet;

fn main() {
    let specs = [
        "k8s_openapi::api::core::v1::NamespaceSpec",
        "k8s_openapi::api::core::v1::NodeSpec",
        "k8s_openapi::api::core::v1::PodSpec",
        "k8s_openapi::api::core::v1::ReplicationControllerSpec",
        "k8s_openapi::api::core::v1::ReplicationControllerSpec",
        "k8s_openapi::api::core::v1::ServiceSpec",

        "k8s_openapi::api::apps::v1::DeamonSetSpec",
        "k8s_openapi::api::apps::v1::DeploymentSpec",
        "k8s_openapi::api::apps::v1::ReplicaSetSpec",
        "k8s_openapi::api::apps::v1::StatefulSetSpec",


        "k8s_openapi::api::policy::v1beta1::PodDisruptionBudgetSpec",
    ];

    let lint = build_lint_trait(&specs);
    let ns = build_imports(&specs);

    let source = format!("
{}

{}
", ns, lint);

    write_to(&source.trim(), "../src/linters/lint.rs");

}

fn build_imports(specs: &[&str]) -> String {
    let distinct: HashSet<String> = specs.iter()
        .map(|namespace|  {
            let split: Vec<&str> = namespace.split("::").collect();
            let count = split.len();
            let ns = split[0..count-1].join("::").to_string();

            format!("use {};", ns)
        })
        .collect();

    distinct.iter().cloned().collect::<Vec<String>>().join("\n")
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
        let method_name = format!("{}_{}", version, clean_name);
        let struct_path = format!("{}::{}", version, object);

        spec_str.push_str(&format!("\tfn {}(&self, _{}: &{}) -> Vec<korrecte::report::Finding> {{}}\n", method_name, clean_name, struct_path));
    }


    format!("pub trait Lint {{
{}
}}", spec_str)
}

fn write_to(content: &str, path: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}