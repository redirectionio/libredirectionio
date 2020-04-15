use crate::api::Marker;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    key: String,
    value: String,
    markers: Option<Vec<Marker>>,
}
