use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeaderFilter {
    action: String,
    header: String,
    value: String,
}
