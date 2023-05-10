use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusCodeUpdate {
    pub status_code: u16,
    pub on_response_status_codes: Vec<u16>,
    pub exclude_response_status_codes: bool,
    pub fallback_status_code: u16,
    pub rule_id: Option<String>,
    pub fallback_rule_id: Option<String>,
    pub unit_id: Option<String>,
    pub target_hash: Option<String>,
}

impl StatusCodeUpdate {
    pub fn get_status_code(&self, response_status_code: u16) -> (u16, Option<String>) {
        if response_status_code == 0 && self.on_response_status_codes.is_empty() {
            return (self.status_code, self.rule_id.clone());
        }

        if self.exclude_response_status_codes && !self.on_response_status_codes.contains(&response_status_code) {
            return (self.status_code, self.rule_id.clone());
        }

        if !self.exclude_response_status_codes && self.on_response_status_codes.iter().any(|v| *v == response_status_code) {
            return (self.status_code, self.rule_id.clone());
        }

        if response_status_code != 0 {
            return (self.fallback_status_code, self.fallback_rule_id.clone());
        }

        (0, None)
    }
}
