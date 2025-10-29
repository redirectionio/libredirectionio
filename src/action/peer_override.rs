use serde::{Deserialize, Serialize};

use crate::api::Peer;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PeerOverride {
    pub peer: Peer,
    pub rule_id: Option<String>,
    pub unit_id: Option<String>,
}
