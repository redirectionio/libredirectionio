use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeaderFilter {
    pub action: String,
    pub header: String,
    pub value: String,
    // In 3.0 make this mandatory
    pub id: Option<String>,
    // In 3.0 make this mandatory
    pub target_hash: Option<String>,
}
