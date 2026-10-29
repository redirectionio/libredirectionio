//! A challenge served instead of the page: the browser runs a proof of work and a set of
//! probes, then stores the result in a cookie. The cookie is self-contained, so the
//! agent checks it on every request without keeping any state.
//!
//! Challenge: `1.<expires>.<difficulty>.<nonce>.<signature>`, the signature binding the
//! challenge to the client network and user agent.
//! Cookie: `<challenge>.<probes>.<counter>`, where the SHA-256 of the whole value starts
//! with `difficulty` zero bits.

use std::net::IpAddr;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::{api::Captcha, http::Request};

pub const COOKIE_NAME: &str = "rio_captcha";
const VERSION: &str = "1";
const SIGNATURE_LENGTH: usize = 16;
const MAX_PROBES_LENGTH: usize = 512;
const PAGE: &str = include_str!("challenge.html");

#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    Passed,
    /// No cookie, or one that expired or was issued to another client.
    Missing,
    Rejected,
}

pub fn verify(config: &Captcha, request: &Request, now: i64) -> Verdict {
    if let Some(user_agent) = header(request, "user-agent")
        && user_agent.contains("HeadlessChrome")
    {
        return Verdict::Rejected;
    }

    let mut verdict = Verdict::Missing;

    for value in cookies(request) {
        match verify_cookie(config, request, now, value) {
            Verdict::Passed => return Verdict::Passed,
            Verdict::Rejected => verdict = Verdict::Rejected,
            Verdict::Missing => (),
        }
    }

    verdict
}

pub fn challenge_page(config: &Captcha, request: &Request, now: i64) -> String {
    let difficulty = config.difficulty();
    let expires = now.saturating_add(config.ttl as i64);
    let nonce = format!("{:016x}", rand::random::<u64>());
    let payload = format!("{VERSION}.{expires}.{difficulty}.{nonce}");
    let signature = sign(config, request, &payload);

    PAGE.replace("__CHALLENGE__", &format!("{payload}.{signature}"))
        .replace("__DIFFICULTY__", &difficulty.to_string())
        .replace("__MAX_AGE__", &config.ttl.to_string())
        .replace("__COOKIE__", COOKIE_NAME)
}

fn verify_cookie(config: &Captcha, request: &Request, now: i64, value: &str) -> Verdict {
    let parts = value.split('.').collect::<Vec<_>>();

    let [version, expires, difficulty, nonce, signature, probes, counter] = parts.as_slice() else {
        return Verdict::Missing;
    };

    if *version != VERSION || counter.is_empty() || counter.len() > 20 || !counter.bytes().all(|b| b.is_ascii_digit()) {
        return Verdict::Missing;
    }

    let (Ok(expires), Ok(difficulty)) = (expires.parse::<i64>(), difficulty.parse::<u8>()) else {
        return Verdict::Missing;
    };

    // An expired challenge, or one issued before the ttl or the difficulty were changed.
    if expires <= now || expires > now.saturating_add(config.ttl as i64) || difficulty < config.difficulty() {
        return Verdict::Missing;
    }

    let payload = format!("{version}.{expires}.{difficulty}.{nonce}");

    if !signature_matches(config, request, &payload, signature) {
        return Verdict::Missing;
    }

    if leading_zero_bits(&Sha256::digest(value.as_bytes())) < u32::from(difficulty) {
        return Verdict::Rejected;
    }

    match Probes::decode(probes) {
        Some(probes) if !probes.is_automated() => Verdict::Passed,
        _ => Verdict::Rejected,
    }
}

fn sign(config: &Captcha, request: &Request, payload: &str) -> String {
    let mac = mac(config, request, payload).finalize().into_bytes();

    hex(&mac[..SIGNATURE_LENGTH])
}

fn signature_matches(config: &Captcha, request: &Request, payload: &str, signature: &str) -> bool {
    let Some(signature) = unhex(signature) else {
        return false;
    };

    signature.len() == SIGNATURE_LENGTH && mac(config, request, payload).verify_truncated_left(&signature).is_ok()
}

fn mac(config: &Captcha, request: &Request, payload: &str) -> Hmac<Sha256> {
    let mut mac = Hmac::<Sha256>::new_from_slice(config.secret.as_bytes()).expect("hmac accepts keys of any size");

    mac.update(payload.as_bytes());
    mac.update(b"|");
    mac.update(network(request.remote_addr).as_bytes());
    mac.update(b"|");
    mac.update(header(request, "user-agent").unwrap_or_default().as_bytes());

    mac
}

/// The client address is truncated so that a browser keeps its clearance when it moves
/// inside its network, as mobile and dual-stack clients do.
fn network(addr: Option<IpAddr>) -> String {
    match addr {
        Some(IpAddr::V4(ip)) => {
            let [a, b, c, _] = ip.octets();
            format!("{a}.{b}.{c}")
        }
        Some(IpAddr::V6(ip)) => {
            let [a, b, c, d, ..] = ip.segments();
            format!("{a:x}:{b:x}:{c:x}:{d:x}")
        }
        None => String::new(),
    }
}

fn header<'a>(request: &'a Request, name: &str) -> Option<&'a str> {
    request
        .headers
        .iter()
        .find(|header| header.name.eq_ignore_ascii_case(name))
        .map(|header| header.value.as_str())
}

fn cookies(request: &Request) -> impl Iterator<Item = &str> {
    request
        .headers
        .iter()
        .filter(|header| header.name.eq_ignore_ascii_case("cookie"))
        .flat_map(|header| header.value.split(';'))
        .filter_map(|cookie| cookie.trim().split_once('='))
        .filter(|(name, _)| *name == COOKIE_NAME)
        .map(|(_, value)| value)
}

fn leading_zero_bits(digest: &[u8]) -> u32 {
    let mut bits = 0;

    for byte in digest {
        bits += byte.leading_zeros();

        if *byte != 0 {
            break;
        }
    }

    bits
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn unhex(value: &str) -> Option<Vec<u8>> {
    if !value.len().is_multiple_of(2) {
        return None;
    }

    (0..value.len())
        .step_by(2)
        .map(|i| value.get(i..i + 2).and_then(|byte| u8::from_str_radix(byte, 16).ok()))
        .collect()
}

/// What the page found in the browser. Only booleans travel, never the values they were
/// computed from (renderer, languages, screen...), and nothing is kept once checked.
#[derive(Deserialize, Debug, Default)]
struct Probes {
    /// Set once every probe ran, a page interrupted halfway sends nothing usable.
    v: u8,
    /// Pointer, touch, key or scroll events seen while the challenge ran.
    #[serde(default)]
    it: u32,
    /// `navigator.webdriver` is true.
    #[serde(default)]
    wd: bool,
    /// The user agent or its client hints name a headless browser.
    #[serde(default)]
    hl: bool,
    /// Globals or attributes left by automation tools.
    #[serde(default)]
    au: bool,
    /// Native functions of the browser were replaced by scripts.
    #[serde(default)]
    nt: bool,
    /// A debugging protocol client inspects the page.
    #[serde(default)]
    dp: bool,
    /// WebGL renders in software.
    #[serde(default)]
    sw: bool,
    /// The user agent contradicts the platform or the engine.
    #[serde(default)]
    ua: bool,
    /// No preferred language.
    #[serde(default)]
    lg: bool,
    /// The screen or the window has no size.
    #[serde(default)]
    sc: bool,
    /// The notification permission contradicts the permissions API.
    #[serde(default)]
    pm: bool,
}

impl Probes {
    fn decode(value: &str) -> Option<Self> {
        if value.len() > MAX_PROBES_LENGTH {
            return None;
        }

        let probes = serde_json::from_slice::<Self>(&URL_SAFE_NO_PAD.decode(value).ok()?).ok()?;

        (probes.v == 1).then_some(probes)
    }

    fn is_automated(&self) -> bool {
        if self.wd || self.hl || self.au || self.nt {
            return true;
        }

        // Each of these also happens in genuine browsers (a virtual machine, an open
        // devtools, a privacy extension), only their accumulation is telling.
        let hints = [self.dp, self.sw, self.ua, self.lg, self.sc, self.pm]
            .iter()
            .filter(|hint| **hint)
            .count();

        hints >= 3 || (hints == 2 && self.it == 0)
    }
}

#[cfg(test)]
mod tests;
