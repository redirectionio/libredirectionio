use serde::{Serialize, Deserialize};
use crate::router::rule::Rule;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiAgentRuleResponse {
    #[serde(rename = "hydra:member")]
    pub rules: Vec<Rule>,
}
