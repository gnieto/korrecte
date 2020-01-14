use crate::config::Config;
use crate::kube::api_async::FrozenObjectRepository;
use crate::kube::file::FileObjectRepository;
use crate::kube::{KubeVersion, ObjectRepository};
use crate::linters::evaluator::{Evaluator, SingleEvaluator};
use crate::linters::LintCollection;
use crate::reporting::{Reporter, SingleThreadedReporter};
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub enum ConfigError {
    CouldNotLoadError,
    CouldNotParseError,
}

pub enum ExecutionMode<'a> {
    Api,
    FileSystem(&'a Path, Option<KubeVersion>),
}

#[derive(Default)]
pub struct ExecutionContextBuilder<'a> {
    mode: Option<ExecutionMode<'a>>,
    configuration: Option<Config>,
}

impl<'a> ExecutionContextBuilder<'a> {
    pub fn configuration_from_path(mut self, path: &Path) -> Result<ExecutionContextBuilder<'a>> {
        let config = Self::load_config_from_filesystem(path)?;
        self.configuration = Some(config);

        Ok(self)
    }

    pub fn execution_mode(mut self, mode: ExecutionMode<'a>) -> ExecutionContextBuilder<'a> {
        self.mode = Some(mode);

        self
    }

    pub fn build(self) -> ExecutionContext<'a> {
        ExecutionContext {
            mode: self.mode.unwrap_or(ExecutionMode::Api),
            configuration: self.configuration.unwrap_or_default(),
        }
    }

    fn load_config_from_filesystem(path: &Path) -> Result<Config> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        Ok(toml::from_str(&buffer)?)
    }
}

pub struct ExecutionContext<'a> {
    mode: ExecutionMode<'a>,
    configuration: Config,
}

pub struct Executor<'a> {
    context: ExecutionContext<'a>,
}

impl<'a> Executor<'a> {
    pub fn with_context(context: ExecutionContext<'a>) -> Executor<'a> {
        Executor { context }
    }

    pub fn execute(self) -> Result<impl Reporter> {
        let reporter = SingleThreadedReporter::default();
        let object_repository = self.load_object_repository()?;
        let lints = LintCollection::all(self.context.configuration);
        let evaluator = SingleEvaluator;
        evaluator.evaluate(&reporter, &lints, &*object_repository);

        Ok(reporter)
    }

    fn load_object_repository(&self) -> Result<Box<dyn ObjectRepository>> {
        match &self.context.mode {
            ExecutionMode::FileSystem(path, version) => Ok(Box::new(FileObjectRepository::new(
                Path::new(path),
                version.clone(),
            )?)),
            ExecutionMode::Api => {
                let api = crate::kube::api_async::ApiObjectRepository::new()?;
                Ok(Box::new(FrozenObjectRepository::from(api)))
            }
        }
    }
}
