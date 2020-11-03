use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
    pub value: Option<String>,
}
