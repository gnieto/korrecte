use crate::linters::LintSpec;
use kube::api::ObjectMeta;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use serde::{Deserialize, Serialize};
pub use korrecte_shared::reporting::Finding;

pub trait Reporter {
    fn report(&self, finding: Finding);
    fn findings(&self) -> Vec<Finding>;
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
