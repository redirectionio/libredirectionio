use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogOverride {
    pub log_override: bool,
    pub rule_id: Option<String>,
    pub on_response_status_codes: Vec<u16>,
    pub fallback_log_override: Option<bool>,
    pub fallback_rule_id: Option<String>,
}

impl LogOverride {
    pub fn get_log_override(&self, response_status_code: u16) -> (Option<bool>, Option<String>) {
        if self.on_response_status_codes.iter().any(|v| *v == response_status_code) {
            return (Some(self.log_override), self.rule_id.clone());
        }

        return (self.fallback_log_override, self.fallback_rule_id.clone());
    }
}
