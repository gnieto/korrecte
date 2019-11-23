use crate::KubeClientError;
use std::fs::File;
use std::io::Read;

pub fn inline_or_file(
    inline: &Option<Vec<u8>>,
    path: &Option<String>,
) -> Result<Vec<u8>, KubeClientError> {
    match inline {
        Some(data) => return Ok(data.clone()),
        _ => (),
    }

    match path {
        Some(path) => return load_from_file(path),
        _ => (),
    }

    Err(KubeClientError::LocalConfigNotFound)
}

pub fn load_from_file(path: &String) -> Result<Vec<u8>, KubeClientError> {
    let mut file = File::open(path).map_err(|_| KubeClientError::LocalConfigNotFound)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .map_err(|_| KubeClientError::LocalConfigNotFound)?;

    Ok(contents)
}
