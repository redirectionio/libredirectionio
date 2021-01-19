use crate::action::Action;
use crate::http::{Header, Request};
use serde::{Deserialize, Serialize};
use std::iter::FromIterator;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Log {
    code: u16,
    to: String,
    time: u64,
    proxy: String,
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
    pub fn from_proxy(request: &Request, code: u16, response_headers: &[Header], action: Option<&Action>, proxy: &str, time: u64) -> Log {
        let mut location = None;
        let mut user_agent = None;
        let mut referer = None;
        let mut content_type = None;

        for header in &request.headers {
            if header.name.to_lowercase() == "user-agent" {
                user_agent = Some(header.value.clone())
            }

            if header.name.to_lowercase() == "referer" {
                referer = Some(header.value.clone())
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
            rule_ids: match action {
                None => None,
                Some(action) => match action.rules_applied.as_ref() {
                    None => Some(action.rule_ids.clone()),
                    Some(ids) => Some(Vec::from_iter(ids.clone())),
                },
            },
            url: request.path_and_query.original.clone(),
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
            to: location.unwrap_or_else(|| "".to_string()),
        }
    }
}
