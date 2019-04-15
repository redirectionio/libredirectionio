use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Redirect {
    pub status: u16,
    pub target: String,
}
