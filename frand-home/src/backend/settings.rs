use anyhow::Result;

pub struct Settings;

impl Settings {
    pub fn log4rs() -> Result<String> { Ok(dotenv::var("FRAND_HOME_LOG4RS")?) }
    pub fn local_mode() -> Result<bool> { Ok(dotenv::var("FRAND_HOME_LOCAL_MODE")?.parse()?) }
    pub fn server_port() -> Result<u16> { Ok(dotenv::var("FRAND_HOME_SERVER_PORT")?.parse()?) }
    pub fn tls_cert() -> Result<String> { Ok(dotenv::var("FRAND_HOME_TLS_CERT")?) }
    pub fn tls_key() -> Result<String> { Ok(dotenv::var("FRAND_HOME_TLS_KEY")?) }
}