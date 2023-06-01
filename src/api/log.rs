use crate::action::Action;
use crate::http::{Addr, Header, Request, TrustedProxies};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Log {
    code: u16,
    to: String,
    time: u64,
    proxy: String,
    ips: Option<Vec<String>>,
    from: FromLog,
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
        let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Err(_) => 0,
            Ok(time) => time.as_millis() as u64,
        };

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
        }
    }

    pub fn from_proxy(
        request: &Request,
        code: u16,
        response_headers: &[Header],
        action: Option<&Action>,
        proxy: &str,
        time: u64,
        client_ip: &str,
        trusted_proxies: Option<&TrustedProxies>,
    ) -> Log {
        let mut location = None;
        let mut user_agent = None;
        let mut referer = None;
        let mut content_type = None;
        let mut ips = Vec::new();

        match client_ip.parse::<Addr>() {
            Ok(addr) => ips.push(addr.addr),
            Err(()) => {
                log::warn!("cannot parse ip address {}, skipping", client_ip);
            }
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

        let untrusted_ips = match trusted_proxies {
            Some(trusted_proxies) => trusted_proxies.remove_trusted_ips(ips),
            None => ips,
        };

        for header in response_headers {
            if header.name.to_lowercase() == "location" {
                location = Some(header.value.clone())
            }

            if header.name.to_lowercase() == "content-type" {
                content_type = Some(header.value.clone())
            }
        }

        let from = FromLog {
            rule_ids: action.map(|a| a.get_applied_rule_ids()),
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
            time,
            ips: Some(untrusted_ips.iter().map(|ip| ip.to_string()).collect()),
            to: location.unwrap_or_default(),
        }
    }
}
