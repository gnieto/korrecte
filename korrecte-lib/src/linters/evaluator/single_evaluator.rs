use crate::config::KorrecteConfig;
use crate::linters::evaluator::{Context, Evaluator};
use crate::linters::{KubeObjectType, LintList};

pub struct SingleEvaluator;

impl Evaluator for SingleEvaluator {
    fn evaluate<'a>(&self, context: &'a Context<'a>, list: &LintList) {
        for lint in list.iter() {
            for object in context.repository.iter() {
                if !Self::needs_linting(context.config, object) {
                    continue;
                }

                lint.object(&object, &context);
            }
        }
    }
}

impl SingleEvaluator {
    fn needs_linting(config: &KorrecteConfig, object: &KubeObjectType) -> bool {
        let namespace = object
            .metadata()
            .and_then(|m| m.namespace.clone())
            .unwrap_or_else(|| "".to_string());

        if !config.ignored_namespaces.is_empty() {
            return !config.ignored_namespaces.contains(&namespace);
        }

        if !config.allowed_namespaces.is_empty() {
            return config.allowed_namespaces.contains(&namespace);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::config::KorrecteConfig;
    use crate::linters::evaluator::SingleEvaluator;
    use crate::linters::KubeObjectType;

    #[test]
    fn it_is_ignored_if_it_is_on_ignored_namespaces() {
        let mut cfg = KorrecteConfig::default();
        cfg.ignored_namespaces = vec!["somenamespace".to_string()];
        let object = create_obj_with_metadata();

        let allowed = SingleEvaluator::needs_linting(&cfg, &object);

        assert_eq!(false, allowed);
    }

    #[test]
    fn it_is_allowed_if_not_in_ignored_namespaces() {
        let mut cfg = KorrecteConfig::default();
        cfg.ignored_namespaces = vec!["not-ignored".to_string()];
        let object = create_obj_with_metadata();

        let allowed = SingleEvaluator::needs_linting(&cfg, &object);

        assert_eq!(true, allowed);
    }

    #[test]
    fn it_is_allowed_if_it_is_on_allowed_namespaces() {
        let mut cfg = KorrecteConfig::default();
        cfg.allowed_namespaces = vec!["somenamespace".to_string()];
        let object = create_obj_with_metadata();

        let allowed = SingleEvaluator::needs_linting(&cfg, &object);

        assert_eq!(true, allowed);
    }

    #[test]
    fn it_is_ignored_if_it_is_not_on_allowed_namespaces() {
        let mut cfg = KorrecteConfig::default();
        cfg.allowed_namespaces = vec!["accepted-namespace".to_string()];
        let object = create_obj_with_metadata();

        let allowed = SingleEvaluator::needs_linting(&cfg, &object);

        assert_eq!(false, allowed);
    }

    #[test]
    fn it_is_allowed_if_lists_are_empty() {
        let cfg = KorrecteConfig::default();
        let object = create_obj_with_metadata();

        let allowed = SingleEvaluator::needs_linting(&cfg, &object);

        assert_eq!(true, allowed);
    }

    fn create_obj_with_metadata() -> KubeObjectType {
        let obj = r#"
apiVersion: policy/v1beta1
kind: PodDisruptionBudget
metadata:
  name: missconfigured-pdb
  namespace: somenamespace
spec:
  maxUnavailable: 1
  selector:
    matchLabels:
      app: pdb
        "#;

        KubeObjectType::from_yaml(&obj, "policy/v1beta1", "PodDisruptionBudget").unwrap()
    }
}
