use std::collections::HashMap;
#[cfg(feature = "kubeobjects")]
use kube::api::ObjectMeta;
use serde::Serialize;
use serde::Deserialize;

#[derive(Clone, Serialize, Deserialize)]
pub struct Finding {
    spec: LintSpec,
    name: String,
    namespace: Option<String>,
    // TODO: Think about a better data structure
    lint_metadata: HashMap<String, String>,
}

impl Finding {

    #[cfg(feature = "kubeobjects")]
    pub fn from_object_metadata(spec: LintSpec, object_metadata: ObjectMeta) -> Self {
        Finding {
            spec,
            name: object_metadata.name,
            namespace: object_metadata.namespace,
            lint_metadata: HashMap::new(),
        }
    }

    pub fn new(spec: LintSpec, name: String, namespace: Option<String>) -> Self {
        Finding {
            spec,
            name,
            namespace,
            lint_metadata: HashMap::new(),
        }
    }

    pub fn add_metadata<K: ToString, V: ToString>(mut self, key: K, value: V) -> Self {
        self.lint_metadata
            .insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_metadata(mut self, lint_meta: HashMap<String, String>) -> Self {
        self.lint_metadata = lint_meta;
        self
    }

    #[allow(unused)]
    pub fn spec(&self) -> &LintSpec {
        &self.spec
    }

    #[allow(unused)]
    pub fn lint_metadata(&self) -> &HashMap<String, String> {
        &self.lint_metadata
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn namespace(&self) -> &Option<String> {
        &self.namespace
    }
}


#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub enum Group {
    Audit,
    Configuration,
    Security,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct LintSpec {
    pub group: Group,
    pub name: String,
}