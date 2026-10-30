use std::{cell::RefCell, rc::Rc, str::FromStr};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};

use super::{Verdict, challenge_page, leading_zero_bits, verify};
use crate::{
    action::{Action, UnitTrace},
    api::{Captcha, CaptchaProvider, DEFAULT_CAPTCHA_COOKIE_NAME, Rule},
    http::Request,
    router::Router,
};

const NOW: i64 = 1_800_000_000;
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0 Safari/537.36";

fn config() -> Captcha {
    Captcha {
        provider: CaptchaProvider::ProofOfWork,
        secret: "secret".to_string(),
        difficulty: 8,
        ttl: 3600,
        cookie_name: None,
        id: Some("captcha-unit".to_string()),
    }
}

fn request(ip: &str, user_agent: &str, cookie: Option<&str>) -> Request {
    request_with_cookie(ip, user_agent, DEFAULT_CAPTCHA_COOKIE_NAME, cookie)
}

fn request_with_cookie(ip: &str, user_agent: &str, cookie_name: &str, cookie: Option<&str>) -> Request {
    let mut request = Request::from_str("http://example.org/page").unwrap();
    request.remote_addr = Some(ip.parse().unwrap());
    request.created_at = chrono::DateTime::from_timestamp(NOW, 0);
    request.add_header("User-Agent".to_string(), user_agent.to_string(), false);

    if let Some(cookie) = cookie {
        request.add_header("Cookie".to_string(), format!("other=1; {cookie_name}={cookie}"), false);
    }

    request
}

fn challenge(config: &Captcha, request: &Request) -> String {
    let page = challenge_page(config, request, NOW);
    let start = page.find("CHALLENGE = \"").unwrap() + "CHALLENGE = \"".len();

    page[start..start + page[start..].find('"').unwrap()].to_string()
}

/// Does what the page does in the browser.
fn solve(challenge: &str, probes: &str, difficulty: u8) -> String {
    let prefix = format!("{challenge}.{}.", URL_SAFE_NO_PAD.encode(probes));

    (0u64..)
        .map(|counter| format!("{prefix}{counter}"))
        .find(|value| leading_zero_bits(&Sha256::digest(value.as_bytes())) >= u32::from(difficulty))
        .unwrap()
}

fn solved(probes: &str) -> String {
    let config = config();

    solve(
        &challenge(&config, &request("192.0.2.10", USER_AGENT, None)),
        probes,
        config.difficulty,
    )
}

fn verdict(ip: &str, user_agent: &str, cookie: &str) -> Verdict {
    verify(&config(), &request(ip, user_agent, Some(cookie)), NOW + 10)
}

#[test]
fn without_cookie_the_challenge_is_missing() {
    assert_eq!(verify(&config(), &request("192.0.2.10", USER_AGENT, None), NOW), Verdict::Missing);
}

#[test]
fn a_solved_challenge_passes() {
    assert_eq!(verdict("192.0.2.10", USER_AGENT, &solved(r#"{"v":1}"#)), Verdict::Passed);
}

#[test]
fn the_clearance_follows_the_client_inside_its_network_only() {
    let cookie = solved(r#"{"v":1}"#);

    assert_eq!(verdict("192.0.2.200", USER_AGENT, &cookie), Verdict::Passed);
    assert_eq!(verdict("192.0.3.10", USER_AGENT, &cookie), Verdict::Missing);
    assert_eq!(verdict("192.0.2.10", "curl/8.0", &cookie), Verdict::Missing);
}

#[test]
fn an_expired_clearance_is_missing() {
    let cookie = solved(r#"{"v":1}"#);

    assert_eq!(
        verify(&config(), &request("192.0.2.10", USER_AGENT, Some(&cookie)), NOW + 3600),
        Verdict::Missing
    );
}

#[test]
fn raising_the_difficulty_invalidates_previous_clearances() {
    let cookie = solved(r#"{"v":1}"#);
    let config = Captcha {
        difficulty: 12,
        ..config()
    };

    assert_eq!(
        verify(&config, &request("192.0.2.10", USER_AGENT, Some(&cookie)), NOW + 10),
        Verdict::Missing
    );
}

#[test]
fn a_tampered_cookie_is_refused() {
    let cookie = solved(r#"{"v":1}"#);
    let mut parts = cookie.split('.').map(str::to_string).collect::<Vec<_>>();

    parts[5] = URL_SAFE_NO_PAD.encode(r#"{"v":1,"it":5}"#);
    assert_ne!(verdict("192.0.2.10", USER_AGENT, &parts.join(".")), Verdict::Passed);

    let mut parts = cookie.split('.').map(str::to_string).collect::<Vec<_>>();
    parts[1] = (NOW + 3000).to_string();
    assert_eq!(verdict("192.0.2.10", USER_AGENT, &parts.join(".")), Verdict::Missing);

    assert_eq!(verdict("192.0.2.10", USER_AGENT, "garbage"), Verdict::Missing);
}

#[test]
fn incomplete_probes_are_refused() {
    assert_eq!(verdict("192.0.2.10", USER_AGENT, &solved(r#"{"it":3}"#)), Verdict::Rejected);
    assert_eq!(verdict("192.0.2.10", USER_AGENT, &solved("not json")), Verdict::Rejected);
}

#[test]
fn automation_is_refused() {
    for probes in [
        r#"{"v":1,"wd":true}"#,
        r#"{"v":1,"hl":true}"#,
        r#"{"v":1,"au":true}"#,
        r#"{"v":1,"nt":true}"#,
    ] {
        assert_eq!(verdict("192.0.2.10", USER_AGENT, &solved(probes)), Verdict::Rejected, "{probes}");
    }
}

#[test]
fn hints_only_count_together() {
    assert_eq!(verdict("192.0.2.10", USER_AGENT, &solved(r#"{"v":1,"sw":true}"#)), Verdict::Passed);
    assert_eq!(
        verdict("192.0.2.10", USER_AGENT, &solved(r#"{"v":1,"sw":true,"lg":true,"it":4}"#)),
        Verdict::Passed
    );
    assert_eq!(
        verdict("192.0.2.10", USER_AGENT, &solved(r#"{"v":1,"sw":true,"lg":true}"#)),
        Verdict::Rejected
    );
    assert_eq!(
        verdict("192.0.2.10", USER_AGENT, &solved(r#"{"v":1,"sw":true,"lg":true,"dp":true,"it":4}"#)),
        Verdict::Rejected
    );
}

#[test]
fn a_headless_user_agent_is_refused() {
    let user_agent = USER_AGENT.replace("Chrome", "HeadlessChrome");

    assert_eq!(verify(&config(), &request("192.0.2.10", &user_agent, None), NOW), Verdict::Rejected);
}

#[test]
fn the_cookie_name_can_be_configured() {
    let config = Captcha {
        cookie_name: Some("bot_check".to_string()),
        ..config()
    };
    let cookie = solved(r#"{"v":1}"#);

    assert!(challenge_page(&config, &request("192.0.2.10", USER_AGENT, None), NOW).contains("COOKIE = \"bot_check\""));
    assert_eq!(
        verify(
            &config,
            &request_with_cookie("192.0.2.10", USER_AGENT, "bot_check", Some(&cookie)),
            NOW + 10
        ),
        Verdict::Passed
    );
    assert_eq!(
        verify(&config, &request("192.0.2.10", USER_AGENT, Some(&cookie)), NOW + 10),
        Verdict::Missing
    );
}

#[test]
fn an_unsafe_cookie_name_falls_back_to_the_default_one() {
    for name in ["", "a\";alert(1);//", "with space", "semi;colon", &"x".repeat(65)] {
        let config = Captcha {
            cookie_name: Some(name.to_string()),
            ..config()
        };

        assert_eq!(config.cookie_name(), DEFAULT_CAPTCHA_COOKIE_NAME, "{name}");
    }
}

fn router() -> Router<Rule> {
    let rule = Rule::from_json(
        r#"{"id":"captcha-rule","rank":0,"source":{"path":"/page"},"target":null,"status_code":null,"body_filters":null,"header_filters":null,"log_override":null,"peer_override":null,"reset":null,"stop":null,"examples":null,"redirect_unit_id":null,"configuration_log_unit_id":null,"configuration_reset_unit_id":null,"peer_unit_id":null,"target_hash":null,"captcha":{"secret":"secret","difficulty":8,"ttl":3600,"id":"captcha-unit"}}"#,
    )
    .unwrap();
    let mut router = Router::<Rule>::default();
    router.insert(rule);

    router
}

fn action_for(request: &Request) -> Action {
    let router = router();
    Action::from_routes_rule(router.match_request(request), request, None)
}

#[test]
fn the_rule_answers_with_the_challenge() {
    let request = request("192.0.2.10", USER_AGENT, None);
    let mut action = action_for(&request);
    let trace = Rc::new(RefCell::new(UnitTrace::default()));

    assert_eq!(action.get_status_code(0, Some(trace.clone())), 403);
    assert_eq!(action.get_log_tags(403, None), vec!["captcha:challenged"]);

    let headers = action.filter_headers(Vec::new(), 403, false, None);
    assert!(headers.iter().any(|h| h.name == "Cache-Control" && h.value == "no-store, private"));

    let mut body = action.create_filter_body(403, &headers, None).unwrap();
    let mut page = body.filter(Vec::new(), None);
    page.extend(body.end(None));
    let page = String::from_utf8(page).unwrap();

    assert!(page.contains("var CHALLENGE = \"1."));
    for placeholder in ["__CHALLENGE__", "__DIFFICULTY__", "__MAX_AGE__", "__COOKIE__"] {
        assert!(!page.contains(placeholder), "{placeholder} is replaced");
    }
}

#[test]
fn the_rule_lets_a_cleared_browser_through() {
    let request = request("192.0.2.10", USER_AGENT, Some(&solved(r#"{"v":1}"#)));
    let mut action = action_for(&request);

    assert_eq!(action.get_status_code(0, None), 0);
    assert_eq!(action.get_log_tags(200, None), vec!["captcha:passed"]);
}
