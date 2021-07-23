use crate::action::Action;
use crate::http::{Header, Request};
use serde::{Deserialize, Serialize};

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

impl Log {
    pub fn from_proxy(
        request: &Request,
        code: u16,
        response_headers: &[Header],
        action: Option<&Action>,
        proxy: &str,
        time: u64,
        client_ip: &str,
    ) -> Log {
        let mut location = None;
        let mut user_agent = None;
        let mut referer = None;
        let mut content_type = None;
        let mut ips = vec![client_ip.to_string()];

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
                    ips.push(forwarded_ip.trim().to_string());
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
            ips: Some(ips),
            to: location.unwrap_or_else(|| "".to_string()),
        }
    }
}
