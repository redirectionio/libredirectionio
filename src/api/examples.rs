use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Example {
    pub url: String,
    pub method: Option<String>,
    pub headers: Option<Vec<ExampleHeader>>,
    pub datetime: Option<String>,
    pub ip_address: Option<String>,
    pub response_status_code: Option<u16>,
    pub must_match: bool,
    pub unit_ids_applied: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExampleHeader {
    pub name: String,
    pub value: String,
}

impl Example {
    pub fn with_url(&self, url: String) -> Example {
        let mut new_exemple = self.clone();
        new_exemple.url = url;
        new_exemple
    }
}
