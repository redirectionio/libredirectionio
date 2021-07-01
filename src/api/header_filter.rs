use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeaderFilter {
    pub action: String,
    pub header: String,
    pub value: String,
}
