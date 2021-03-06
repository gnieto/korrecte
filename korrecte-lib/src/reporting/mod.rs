use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

pub trait Reporter {
    fn report(&self, finding: Finding);
    fn findings(&self) -> Vec<Finding>;
}

#[derive(Clone, Serialize, Debug)]
pub struct Finding {
    lint_name: String,
    name: String,
    namespace: Option<String>,
    // TODO: Think about a better data structure
    lint_metadata: HashMap<String, String>,
}

impl Finding {
    pub fn new(lint_name: &str, object_metadata: Option<ObjectMeta>) -> Self {
        let metadata = object_metadata.unwrap_or_default();

        Finding {
            lint_name: lint_name.to_string(),
            name: metadata.name.unwrap_or_default(),
            namespace: metadata.namespace.clone(),
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

    pub fn lint_name(&self) -> &String {
        &self.lint_name
    }

    #[allow(unused)]
    pub fn lint_metadata(&self) -> &HashMap<String, String> {
        &self.lint_metadata
    }

    #[allow(unused)]
    pub fn name(&self) -> &String {
        &self.name
    }

    #[allow(unused)]
    pub fn namespace(&self) -> &Option<String> {
        &self.namespace
    }
}

#[derive(Default, Clone)]
pub struct SingleThreadedReporter {
    findings: Rc<RefCell<Vec<Finding>>>,
}

impl Reporter for SingleThreadedReporter {
    fn report(&self, finding: Finding) {
        let mut guard = self.findings.borrow_mut();
        guard.push(finding);
    }

    fn findings(&self) -> Vec<Finding> {
        let guard = self.findings.borrow();
        guard.deref().clone()
    }
}
