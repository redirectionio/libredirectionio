use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum IpConstraint {
    InRange(String),
    NotInRange(String),
    NotOneOf(Vec<String>),
    #[serde(untagged)]
    Other(serde_json::Value),
}
