use serde::{Deserialize, Serialize};

use crate::api::{DateTimeConstraint, Header, IpConstraint};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    pub scheme: Option<String>,
    pub host: Option<String>,
    pub ips: Option<Vec<IpConstraint>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub datetime: Option<Vec<DateTimeConstraint>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub time: Option<Vec<DateTimeConstraint>>,
    pub path: String,
    pub query: Option<String>,
    pub headers: Option<Vec<Header>>,
    pub methods: Option<Vec<String>>,
    pub exclude_methods: Option<bool>,
    pub response_status_codes: Option<Vec<u16>>,
    pub exclude_response_status_codes: Option<bool>,
    pub sampling: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub weekdays: Option<Vec<String>>,
}
