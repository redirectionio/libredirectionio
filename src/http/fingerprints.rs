use serde::{Deserialize, Serialize};

const FINGERPRINT_MAX_LENGTH: usize = 256;

/// Fingerprints of the client connection, computed by the layer which terminated it.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Fingerprints {
    /// JA4 of the TLS ClientHello
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<String>,
    /// p0f signature of the TCP SYN
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<String>,
    /// HTTP/2 fingerprint, in the Akamai format
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http2: Option<String>,
}

impl Fingerprints {
    pub fn is_empty(&self) -> bool {
        self.tls.is_none() && self.tcp.is_none() && self.http2.is_none()
    }

    /// Set the fingerprint named `tls`, `tcp` or `http2`, returns false when the name is unknown or
    /// the value is empty, longer than 256 bytes or not printable ASCII: an empty value leaves the
    /// fingerprint unset.
    pub fn set(&mut self, name: &str, value: &str) -> bool {
        let slot = match name {
            "tls" => &mut self.tls,
            "tcp" => &mut self.tcp,
            "http2" => &mut self.http2,
            _ => return false,
        };
        let value = value.trim();

        if value.is_empty() || value.len() > FINGERPRINT_MAX_LENGTH || !value.bytes().all(|byte| byte.is_ascii_graphic() || byte == b' ') {
            return false;
        }

        *slot = Some(value.to_string());

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Fingerprints;

    #[test]
    fn set_a_known_fingerprint() {
        let mut fingerprints = Fingerprints::default();

        assert!(fingerprints.set("tls", " t13d1516h2_8daaf6152771_e5627efa2ab1 "));
        assert_eq!(fingerprints.tls.as_deref(), Some("t13d1516h2_8daaf6152771_e5627efa2ab1"));
        assert!(!fingerprints.is_empty());
    }

    #[test]
    fn ignore_an_unknown_or_invalid_fingerprint() {
        let mut fingerprints = Fingerprints::default();

        assert!(!fingerprints.set("ja3", "771,4865"));
        assert!(!fingerprints.set("tcp", ""));
        assert!(!fingerprints.set("tcp", "4:64\r\n+0"));
        assert!(!fingerprints.set("http2", &"1".repeat(257)));
        assert!(fingerprints.is_empty());
        assert_eq!(serde_json::to_string(&fingerprints).unwrap(), "{}");
    }
}
