use crate::api::Header;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    pub scheme: Option<String>,
    pub host: Option<String>,
    pub path: String,
    pub query: Option<String>,
    pub headers: Option<Vec<Header>>,
    pub methods: Option<Vec<String>>,
}
