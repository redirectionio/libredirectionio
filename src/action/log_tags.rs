use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogTags {
    pub tags: Vec<String>,
    pub rule_id: Option<String>,
    pub on_response_status_codes: Vec<u16>,
    pub exclude_response_status_codes: bool,
    pub unit_id: Option<String>,
}

impl LogTags {
    pub fn applies_to(&self, response_status_code: u16) -> bool {
        if self.on_response_status_codes.is_empty() {
            return true;
        }

        self.exclude_response_status_codes != self.on_response_status_codes.contains(&response_status_code)
    }
}
