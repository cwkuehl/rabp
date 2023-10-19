use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_host")]
    pub host: String,
    pub port: u16,
    pub client_origin_url: String,
    #[serde(default = "default_tls_certs")]
    pub tls_certs: String,
    #[serde(default = "default_tls_key")]
    pub tls_key: String,
    pub sqlite_db: String,
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_tls_certs() -> String {
    "".to_string()
}

fn default_tls_key() -> String {
    "".to_string()
}

impl Default for Config {
    fn default() -> Self {
        envy::from_env::<Config>().expect("Provide missing environment variables for Config")
    }
}

#[derive(Serialize)]
pub struct ErrorMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    pub message: String,
}
