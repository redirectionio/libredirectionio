use crate::api::{Header, IpConstraint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    pub scheme: Option<String>,
    pub host: Option<String>,
    pub ips: Option<Vec<IpConstraint>>,
    pub path: String,
    pub query: Option<String>,
    pub headers: Option<Vec<Header>>,
    pub methods: Option<Vec<String>>,
    pub response_status_codes: Option<Vec<u16>>,
    pub sampling: Option<u32>,
}
