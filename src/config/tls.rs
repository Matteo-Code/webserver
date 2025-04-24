use std::fs::File;
use std::io::BufReader;
use rustls::pki_types::{CertificateDer as Certificate, PrivateKeyDer as PrivateKey};
use rustls::server::ServerConfig;
use rustls_pemfile::{certs, rsa_private_keys};

pub fn load_tls_config() -> Result<ServerConfig, std::io::Error> {
    let cert_file = &mut BufReader::new(File::open("cert.pem")?);
    let key_file = &mut BufReader::new(File::open("key.pem")?);

    let cert_chain: Vec<Certificate> = certs(cert_file)
        .collect::<Result<Vec<Certificate>, _>>()
        .expect("failed to read certificates")
        .into_iter()
        .map(Certificate::from)
        .collect();

    let keys: Vec<_> = rsa_private_keys(key_file)
        .collect::<Result<_, _>>()
        .expect("failed to read private key");
    let key = PrivateKey::from(keys.into_iter().next().expect("no private key found"));

    Ok(ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .unwrap())
}