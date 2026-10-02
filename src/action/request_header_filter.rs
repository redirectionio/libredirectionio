use serde::{Deserialize, Serialize};

use crate::api::HeaderFilter;

// Framing and hop-by-hop headers: changing them would corrupt the request sent to the
// backend, and the backend host is changed with the switch backend action instead.
const PROTECTED_HEADERS: [&str; 11] = [
    "host",
    "content-length",
    "transfer-encoding",
    "connection",
    "keep-alive",
    "proxy-connection",
    "upgrade",
    "te",
    "trailer",
    "expect",
    "http2-settings",
];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestHeaderFilterAction {
    pub filter: HeaderFilter,
    pub rule_id: Option<String>,
}

pub fn is_protected_request_header(name: &str) -> bool {
    let name = name.trim();

    name.starts_with(':') || PROTECTED_HEADERS.iter().any(|protected| protected.eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::is_protected_request_header;

    #[test]
    fn protected_headers_are_case_insensitive() {
        assert!(is_protected_request_header("Host"));
        assert!(is_protected_request_header("CONTENT-LENGTH"));
        assert!(is_protected_request_header(":authority"));
        assert!(!is_protected_request_header("X-Forwarded-Country"));
        assert!(!is_protected_request_header("Authorization"));
    }
}
