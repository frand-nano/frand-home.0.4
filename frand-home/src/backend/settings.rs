use std::{fs::File, io::BufReader};
use anyhow::{anyhow, Result};
use rustls::{pki_types::PrivateKeyDer, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

pub struct Settings;

impl Settings {
    pub fn log4rs() -> Result<String> { Ok(dotenv::var("FRAND_HOME_LOG4RS")?) }
    pub fn local_mode() -> Result<bool> { Ok(dotenv::var("FRAND_HOME_LOCAL_MODE")?.parse()?) }
    pub fn server_port() -> Result<u16> { Ok(dotenv::var("FRAND_HOME_SERVER_PORT")?.parse()?) }
    pub fn tls_cert() -> Result<String> { Ok(dotenv::var("FRAND_HOME_TLS_CERT")?) }
    pub fn tls_key() -> Result<String> { Ok(dotenv::var("FRAND_HOME_TLS_KEY")?) }

    pub fn read_tls_server_config() -> Result<ServerConfig> {
        let mut certs_file = BufReader::new(File::open(Self::tls_cert()?)?);
        let mut key_file = BufReader::new(File::open(Self::tls_key()?)?);

        let tls_certs = certs(&mut certs_file).collect::<Result<Vec<_>, _>>()?;

        let tls_key = pkcs8_private_keys(&mut key_file).next()
        .ok_or(anyhow!("Found private key file with config, but no TLS private key in that file."))??;
        
        let server_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(tls_certs, PrivateKeyDer::Pkcs8(tls_key))?;

        Ok(server_config)      
    }
}