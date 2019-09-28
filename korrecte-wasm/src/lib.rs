use korrecte_shared::reporting::{Finding, LintSpec, Group};
use korrecte_wasm_macro::korrecte_lint;
use serde_json::from_slice;
use serde_json::to_string;
use std::borrow::Cow;

#[korrecte_lint]
pub fn lint_something(object: Cow<str>) -> Finding {
    let spec = LintSpec {
        group: Group::Security,
        name: "test".to_string(),
    };

    Finding::new(
        spec,
        "aaa".to_string(),
        Some("bbb".to_string()),
    )
}