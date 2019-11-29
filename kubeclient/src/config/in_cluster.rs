use crate::config::ClusterConfig;
use anyhow::{Result, Error, anyhow};
use openssl::pkcs12::Pkcs12;

pub fn load_incluster() -> Result<A> {
    if false {
        return Ok(A{});
    }

    Err(anyhow!("In cluster config loading not yet implemented"))
}

pub struct A {

}

impl ClusterConfig for A {
    fn default_namespace(&self) -> Option<&String> {
        unimplemented!()
    }

    fn base_uri(&self) -> &String {
        unimplemented!()
    }

    fn token(&self) -> Option<&String> {
        unimplemented!()
    }

    fn certificate_authority(&self) -> Result<Vec<u8>> {
        unimplemented!()
    }

    fn skip_authority(&self) -> bool {
        unimplemented!()
    }

    fn identity(&self) -> Option<Pkcs12> {
        unimplemented!()
    }
}