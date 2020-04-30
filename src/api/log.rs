use serde::{Deserialize, Serialize};
use crate::http::{Request, Header};
use crate::action::Action;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Log {
    code: u16,
    to: Option<String>,
    time: i64,
    proxy: String,
    from: FromLog,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FromLog {
    rule_id: Option<String>,
    url: String,
    method: Option<String>,
    scheme: Option<String>,
    host: Option<String>,
    referer: Option<String>,
    user_agent: Option<String>,
}

impl Log {
    pub fn from_proxy(request: &Request, code: u16, response_headers: Vec<Header>, action: &Action, proxy: String, time: i64) -> Log {
        let mut location = None;
        let mut user_agent = None;
        let mut referer = None;

        for header in &request.headers {
            if header.name.to_lowercase() == "user-agent" {
                user_agent = Some(header.value.clone())
            }

            if header.name.to_lowercase() == "referer" {
                referer = Some(header.value.clone())
            }
        }

        for header in &response_headers {
            if header.name.to_lowercase() == "location" {
                location = Some(header.value.clone())
            }
        }

        let from = FromLog {
            rule_id: match action.rule_ids.last() {
                None => None,
                Some(s) => Some(s.clone()),
            },
            url: request.path_and_query.clone(),
            method: request.method.clone(),
            scheme: request.scheme.clone(),
            host: request.host.clone(),
            referer,
            user_agent,
        };

        Log {
            code,
            from,
            proxy,
            time,
            to: location,
        }
    }
}
