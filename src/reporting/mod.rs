use crate::linters::LintSpec;
use kube::api::ObjectMeta;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;

pub trait Reporter {
    fn report(&self, finding: Finding);
    fn findings(&self) -> Vec<Finding>;
}

#[derive(Clone)]
pub struct Finding {
    spec: LintSpec,
    object_metadata: ObjectMeta,
    // TODO: Think about a better data structure
    lint_metadata: HashMap<String, String>,
}

impl Finding {
    pub fn new(spec: LintSpec, object_metadata: ObjectMeta) -> Self {
        Finding {
            spec,
            object_metadata,
            lint_metadata: HashMap::new(),
        }
    }

    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.lint_metadata.insert(key, value);
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
    pub fn object_metadata(&self) -> &ObjectMeta {
        &self.object_metadata
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