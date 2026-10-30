use serde::{Deserialize, Serialize};

pub const DEFAULT_CAPTCHA_DIFFICULTY: u8 = 16;
pub const MAX_CAPTCHA_DIFFICULTY: u8 = 24;
pub const DEFAULT_CAPTCHA_TTL: u64 = 86400;
pub const DEFAULT_CAPTCHA_COOKIE_NAME: &str = "rio_captcha";
const MAX_CAPTCHA_COOKIE_NAME_LENGTH: usize = 64;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Captcha {
    #[serde(default)]
    pub provider: CaptchaProvider,
    /// Signs the challenges, a clearance obtained with one secret is not valid with another.
    pub secret: String,
    /// Number of leading zero bits the proof of work must reach.
    #[serde(default = "default_difficulty")]
    pub difficulty: u8,
    /// How long, in seconds, a solved challenge lets the browser through.
    #[serde(default = "default_ttl")]
    pub ttl: u64,
    /// Name of the cookie holding the solved challenge, defaults to `rio_captcha`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cookie_name: Option<String>,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum CaptchaProvider {
    #[default]
    #[serde(rename = "proof_of_work")]
    ProofOfWork,
    #[serde(other)]
    Unknown,
}

impl Captcha {
    pub fn difficulty(&self) -> u8 {
        self.difficulty.clamp(1, MAX_CAPTCHA_DIFFICULTY)
    }

    /// The name ends up in the script of the challenge page, so anything but a plain token
    /// falls back to the default one.
    pub fn cookie_name(&self) -> &str {
        match self.cookie_name.as_deref() {
            Some(name)
                if !name.is_empty()
                    && name.len() <= MAX_CAPTCHA_COOKIE_NAME_LENGTH
                    && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-') =>
            {
                name
            }
            _ => DEFAULT_CAPTCHA_COOKIE_NAME,
        }
    }
}

fn default_difficulty() -> u8 {
    DEFAULT_CAPTCHA_DIFFICULTY
}

fn default_ttl() -> u64 {
    DEFAULT_CAPTCHA_TTL
}
