use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Peer {
    pub address: String,
    pub sni_host: Option<String>,
    pub request_host: Option<String>,
    pub allow_invalid_certificates: bool,
    pub tls: bool,
}
