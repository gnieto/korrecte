use crate::config::CurrentConfig;
use crate::KubeClientError;
use reqwest::{Certificate, Client, Identity};

pub fn reqwest_client(config: &CurrentConfig) -> Result<Client, KubeClientError> {
    let mut client_builder = Client::builder();

    if let Ok(data) = config.cluster.certificate_authority() {
        let certificate =
            Certificate::from_pem(data.as_slice()).map_err(|_| KubeClientError::Other)?;
        client_builder = client_builder.add_root_certificate(certificate);
    }

    if config.cluster.insecure_skip_tls_verify() {
        client_builder = client_builder.danger_accept_invalid_certs(true);
    }

    if let Ok(identity) = config.auth.identity() {
        let der = identity.to_der().map_err(|_| KubeClientError::SslError)?;
        let id =
            Identity::from_pkcs12_der(der.as_ref(), "").map_err(|_| KubeClientError::SslError)?;
        client_builder = client_builder.identity(id);
    }

    client_builder.build().map_err(|_| KubeClientError::Other)
}
