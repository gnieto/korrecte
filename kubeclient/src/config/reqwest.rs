use crate::config::ClusterConfig;
use anyhow::{Context, Result};
use reqwest::{header, Certificate, Client, Identity};

pub fn reqwest_client(config: &dyn ClusterConfig) -> Result<Client> {
    let mut client_builder = Client::builder();
    let mut headers = header::HeaderMap::new();

    if let Ok(data) = config.certificate_authority() {
        let certificate = Certificate::from_pem(data.as_slice())
            .context("Certificate authority could not be interpreted as PEM")?;

        client_builder = client_builder.add_root_certificate(certificate);
    }

    if config.skip_authority() {
        client_builder = client_builder.danger_accept_invalid_certs(true);
    }

    if let Some(identity) = config.identity() {
        let der = identity
            .to_der()
            .context("Identity file could not be casted to DER")?;
        let id = Identity::from_pkcs12_der(&der, " ")
            .context("Identity data could not be interpreted as pkcs12")?;

        client_builder = client_builder.identity(id);
    }

    if let Some(token) = config.token() {
        let token_header = header::HeaderValue::from_str(&format!("Bearer {}", token))?;

        headers.insert(header::AUTHORIZATION, token_header);
    }

    client_builder
        .default_headers(headers)
        .build()
        .context("Errored while building reqwest client")
}
