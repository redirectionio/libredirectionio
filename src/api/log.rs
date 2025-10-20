use serde::{Deserialize, Serialize};

use crate::{
    action::Action,
    http::{Addr, Header, Request},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Log {
    code: u16,
    to: String,
    time: u128,
    proxy: String,
    ips: Option<Vec<String>>,
    from: FromLog,
    duration: Option<u128>,
    match_duration: Option<u128>,
    proxy_duration: Option<u128>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FromLog {
    #[serde(rename = "ruleIds")]
    rule_ids: Option<Vec<String>>,
    url: String,
    method: Option<String>,
    scheme: Option<String>,
    host: Option<String>,
    referer: Option<String>,
    #[serde(rename = "userAgent")]
    user_agent: Option<String>,
    #[serde(rename = "contentType")]
    content_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LegacyLog {
    status_code: u16,
    host: Option<String>,
    method: Option<String>,
    request_uri: Option<String>,
    user_agent: Option<String>,
    referer: Option<String>,
    scheme: Option<String>,
    use_json: Option<bool>,
    target: Option<String>,
    rule_id: Option<String>,
}

impl Log {
    pub fn from_legacy(legacy: LegacyLog, proxy: String) -> Self {
        let now = chrono::Utc::now().timestamp() as u128;

        Log {
            code: legacy.status_code,
            to: legacy.target.unwrap_or_default(),
            time: now,
            proxy,
            ips: None,
            from: FromLog {
                rule_ids: legacy.rule_id.map(|id| vec![id]),
                url: legacy.request_uri.unwrap_or_default(),
                method: legacy.method,
                scheme: legacy.scheme,
                host: legacy.host,
                referer: legacy.referer,
                user_agent: legacy.user_agent,
                content_type: None,
            },
            duration: None,
            match_duration: None,
            proxy_duration: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_proxy(
        request: &Request,
        code: u16,
        response_headers: &[Header],
        action: Option<&Action>,
        proxy: &str,
        request_start_time: u128,
        action_match_time: u128,
        proxy_response_time: Option<u128>,
        client_ip: &str,
    ) -> Log {
        let mut location = None;
        let mut user_agent = None;
        let mut referer = None;
        let mut content_type = None;
        let mut ips = Vec::new();
        let now = chrono::Utc::now().timestamp_millis() as u128;
        let duration = now.checked_sub(request_start_time);
        let match_duration = action_match_time.checked_sub(request_start_time);
        let proxy_duration = proxy_response_time.and_then(|ms| action_match_time.checked_sub(ms));

        if let Ok(addr) = client_ip.parse::<Addr>() {
            ips.push(addr.addr);
        }

        for header in &request.headers {
            if header.name.to_lowercase() == "user-agent" {
                user_agent = Some(header.value.clone())
            }

            if header.name.to_lowercase() == "referer" {
                referer = Some(header.value.clone())
            }

            if header.name.to_lowercase() == "x-forwarded-for" {
                let forwarded_ips = header.value.split(',');

                for forwarded_ip in forwarded_ips {
                    if let Ok(addr) = forwarded_ip.parse::<Addr>() {
                        ips.push(addr.addr);
                    }
                }
            }

            if header.name.to_lowercase() == "forwarded" {
                for (name, val) in header.value.split(';').flat_map(|val| val.split(',')).flat_map(|pair| {
                    let mut items = pair.trim().splitn(2, '=');
                    Some((items.next()?, items.next()?))
                }) {
                    if name.trim().to_lowercase().as_str() == "for" {
                        let ip = val.trim().trim_start_matches('"').trim_end_matches('"').to_string();

                        if let Ok(ip) = ip.parse::<Addr>() {
                            ips.push(ip.addr);
                        }
                    }
                }
            }
        }

        for header in response_headers {
            if header.name.to_lowercase() == "location" {
                location = Some(header.value.clone())
            }

            if header.name.to_lowercase() == "content-type" {
                content_type = Some(header.value.clone())
            }
        }

        let from = FromLog {
            rule_ids: action.map(|a| a.get_applied_rule_ids().iter().cloned().collect()),
            url: request.path_and_query_skipped.original.clone(),
            method: request.method.clone(),
            scheme: request.scheme.clone(),
            host: request.host.clone(),
            referer,
            user_agent,
            content_type,
        };

        Log {
            code,
            from,
            proxy: proxy.to_string(),
            time: request_start_time,
            ips: Some(ips.iter().map(|ip| ip.to_string()).collect()),
            to: location.unwrap_or_default(),
            duration,
            match_duration,
            proxy_duration,
        }
    }
}
