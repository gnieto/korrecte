use crate::linters::LintSpec;
use kube::api::ObjectMeta;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

pub trait Reporter {
    fn report(&self, finding: Finding);
    fn findings(&self) -> Vec<Finding>;
}

#[derive(Clone, Serialize)]
pub struct Finding {
    spec: LintSpec,
    name: String,
    namespace: Option<String>,
    // TODO: Think about a better data structure
    lint_metadata: HashMap<String, String>,
}

impl Finding {
    pub fn new(spec: LintSpec, object_metadata: ObjectMeta) -> Self {
        Finding {
            spec,
            name: object_metadata.name.clone(),
            namespace: object_metadata.namespace.clone(),
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
