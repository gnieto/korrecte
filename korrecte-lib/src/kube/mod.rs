use crate::linters::KubeObjectType;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

pub mod api_async;
pub mod file;

#[allow(unused)]
pub struct Identifier {
    name: String,
    namespace: Option<String>,
}

impl From<String> for Identifier {
    fn from(s: String) -> Self {
        Identifier {
            name: s,
            namespace: None,
        }
    }
}

impl From<ObjectMeta> for Identifier {
    fn from(meta: ObjectMeta) -> Self {
        Identifier {
            name: meta.name.unwrap(),
            namespace: meta.namespace,
        }
    }
}

#[allow(unused)]
impl Identifier {
    pub fn matches_with(&self, meta: &ObjectMeta) -> bool {
        meta.name.as_ref().cloned().unwrap() == self.name && meta.namespace == self.namespace
    }
}

#[derive(PartialEq, PartialOrd, Clone)]
pub struct KubeVersion {
    major: u16,
    minor: u16,
}

impl KubeVersion {
    pub fn new(major: u16, minor: u16) -> Self {
        KubeVersion { major, minor }
    }

    pub fn maybe_from_str(version: &str) -> Option<KubeVersion> {
        let mut split = version.split('.');
        let major = split.next()?.parse().ok()?;
        let minor = split.next()?.parse().ok()?;

        Some(KubeVersion { major, minor })
    }
}

pub trait ObjectRepository {
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a KubeObjectType> + 'a>;
    fn version(&self) -> Option<&KubeVersion>;
}
