use crate::config::CurrentConfig;
use anyhow::{Context, Result};
use reqwest::{Certificate, Client, Identity};

pub fn reqwest_client(config: &CurrentConfig) -> Result<Client> {
    let mut client_builder = Client::builder();

    if let Ok(data) = config.cluster.certificate_authority() {
        let certificate = Certificate::from_pem(data.as_slice())
            .context("Certificate authority could not be interpreted as PEM")?;

        client_builder = client_builder.add_root_certificate(certificate);
    }

    if config.cluster.insecure_skip_tls_verify() {
        client_builder = client_builder.danger_accept_invalid_certs(true);
    }

    if let Ok(identity) = config.auth.identity() {
        let der = identity
            .to_der()
            .context("Identity file could not be casted to DER")?;
        let id = Identity::from_pkcs12_der(der.as_ref(), "")
            .context("Identity data could not be interpreted as pkcs12")?;

        client_builder = client_builder.identity(id);
    }

    client_builder
        .build()
        .context("Errored while building reqwest client")
}
