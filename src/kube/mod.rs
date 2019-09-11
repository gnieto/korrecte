use kube::api::ObjectMeta;
use crate::linters::KubeObjectType;

pub mod api;
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
            name: meta.name,
            namespace: meta.namespace,
        }
    }
}

#[allow(unused)]
impl Identifier {
    pub fn matches_with(&self, meta: &ObjectMeta) -> bool {
        meta.name == self.name &&
            meta.namespace == self.namespace
    }
}

pub trait ObjectRepository {
    fn all(&self) -> &Vec<KubeObjectType>;
    fn find(&self, id: &Identifier) -> Option<&KubeObjectType>;
}