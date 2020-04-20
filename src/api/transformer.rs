use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transformer {
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub options: Option<HashMap<String, String>>,
}
