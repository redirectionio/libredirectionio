use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusCodeUpdate {
    pub status_code: u16,
    pub on_response_status_codes: Vec<u16>,
    pub fallback_status_code: u16,
}

impl StatusCodeUpdate {
    pub fn get_status_code(&self, response_status_code: u16) -> u16 {
        if response_status_code == 0 && self.on_response_status_codes.is_empty() {
            return self.status_code;
        }

        if self.on_response_status_codes.iter().any(|v| *v == response_status_code) {
            return self.status_code;
        }

        if response_status_code != 0 {
            return self.fallback_status_code;
        }

        0
    }
}
