use crate::api::Rule;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RulesMessage {
    #[serde(rename = "hydra:member")]
    pub rules: Vec<Rule>,
}
