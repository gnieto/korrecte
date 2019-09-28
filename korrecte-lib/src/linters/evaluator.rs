use crate::kube::ObjectRepository;
use crate::linters::LintList;

use crate::reporting::Reporter;

pub struct OneShotEvaluator;

impl OneShotEvaluator {
    pub fn evaluate(
        reporter: &dyn Reporter,
        list: LintList,
        object_repository: &dyn ObjectRepository,
    ) {
        for lint in list.iter() {
            for object in object_repository.all() {
                lint.object(&object, reporter);
            }
        }


        #[cfg(feature = "wasm")]
        use crate::linters::wasm::evaluate;
        #[cfg(feature = "wasm")]
        let finding = evaluate();
        #[cfg(feature = "wasm")]
        reporter.report(finding);
    }
}
