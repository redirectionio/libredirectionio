use crate::api::Transformer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Marker {
    pub name: String,
    pub regex: String,
    pub transformers: Option<Vec<Transformer>>,
}
