use anyhow::{anyhow, Context, Result};
use base64;
use std::fs::File;
use std::io::Read;

pub fn inline_or_file(inline: &Option<String>, path: &Option<String>) -> Result<Vec<u8>> {
    if let Some(data) = inline {
        return base64::decode(data).map_err(|e| e.into());
    }

    if let Some(path) = path {
        return load_from_file(path);
    }

    Err(anyhow!("Both path and inline data are empty"))
}

pub fn load_from_file(path: &str) -> Result<Vec<u8>> {
    let mut file =
        File::open(path).with_context(|| format!("Requested file {} could not be opened", path))?;

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .with_context(|| format!("Requested file {} could not be read", path))?;

    Ok(contents)
}
