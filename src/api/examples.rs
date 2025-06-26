use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Example {
    pub url: String,
    pub method: Option<String>,
    pub headers: Option<Vec<ExampleHeader>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub datetime: Option<String>,
    pub ip_address: Option<String>,
    pub response_status_code: Option<u16>,
    pub must_match: bool,
    pub unit_ids_applied: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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

    pub fn with_method(&self, method: Option<String>) -> Example {
        let mut new_example = self.clone();
        new_example.method = method;
        new_example
    }
}
