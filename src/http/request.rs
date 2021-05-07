use super::header::Header;
use super::query::PathAndQueryWithSkipped;
use crate::router::RouterConfig;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::str::FromStr;
use url::form_urlencoded::parse as parse_query;

const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct Request {
    #[serde(rename = "path_and_query")]
    pub path_and_query_skipped: PathAndQueryWithSkipped,
    #[serde(rename = "path_and_query_v2")]
    pub path_and_query: Option<String>,
    pub host: Option<String>,
    pub scheme: Option<String>,
    pub method: Option<String>,
    pub headers: Vec<Header>,
}

impl FromStr for Request {
    type Err = http::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let http_request = http::Request::<()>::builder().uri(s).method("GET").body(())?;
        let path_and_query_str = match http_request.uri().path_and_query() {
            None => "",
            Some(path_and_query) => path_and_query.as_str(),
        };

        let config = RouterConfig::default();

        Ok(Request::new(
            PathAndQueryWithSkipped::from_config(&config, path_and_query_str),
            path_and_query_str.to_string(),
            match http_request.uri().host() {
                None => None,
                Some(host) => Some(host.to_string()),
            },
            match http_request.uri().scheme_str() {
                None => None,
                Some(scheme) => Some(scheme.to_string()),
            },
            None,
        ))
    }
}

impl Request {
    pub fn new(
        path_and_query_skipped: PathAndQueryWithSkipped,
        path_and_query: String,
        host: Option<String>,
        scheme: Option<String>,
        method: Option<String>,
    ) -> Request {
        Request {
            path_and_query_skipped,
            path_and_query: Some(path_and_query),
            host,
            scheme,
            method,
            headers: Vec::new(),
        }
    }

    pub fn from_config(
        config: &RouterConfig,
        path_and_query: String,
        host: Option<String>,
        scheme: Option<String>,
        method: Option<String>,
    ) -> Request {
        Request {
            path_and_query_skipped: PathAndQueryWithSkipped::from_config(config, path_and_query.as_str()),
            path_and_query: Some(path_and_query),
            host: match host {
                Some(host) => {
                    if config.ignore_host_case {
                        Some(host.to_lowercase())
                    } else {
                        Some(host)
                    }
                }
                None => None,
            },
            scheme,
            method,
            headers: Vec::new(),
        }
    }

    pub fn rebuild_with_config(config: &RouterConfig, request: &Request) -> Self {
        let original_url = match &request.path_and_query {
            Some(str) => str.as_str(),
            None => request.path_and_query_skipped.original.as_str(),
        };

        let path_and_query_skipped = PathAndQueryWithSkipped::from_config(config, original_url);
        let mut headers = Vec::new();

        for header in &request.headers {
            headers.push(Header {
                name: header.name.clone(),
                value: if config.ignore_header_case {
                    header.value.to_lowercase()
                } else {
                    header.value.clone()
                },
            });
        }

        Request {
            path_and_query_skipped,
            path_and_query: Some(original_url.to_string()),
            host: match &request.host {
                Some(host) => {
                    if config.ignore_host_case {
                        Some(host.to_lowercase())
                    } else {
                        Some(host.clone())
                    }
                }
                None => None,
            },
            scheme: request.scheme.clone(),
            method: request.method.clone(),
            headers,
        }
    }

    pub fn add_header(&mut self, name: String, value: String, ignore_case: bool) {
        self.headers.push(Header {
            name,
            value: if ignore_case { value.to_lowercase() } else { value },
        });
    }

    pub fn method(&self) -> &str {
        match &self.method {
            None => "GET",
            Some(method) => method.as_str(),
        }
    }

    pub fn host(&self) -> Option<&str> {
        match &self.host {
            None => None,
            Some(host_str) => Some(host_str.as_str()),
        }
    }

    pub fn scheme(&self) -> Option<&str> {
        match &self.scheme {
            None => None,
            Some(scheme_str) => Some(scheme_str.as_str()),
        }
    }

    pub fn header_exists(&self, name: &str) -> bool {
        let lowercase_name = name.to_lowercase();

        for header in &self.headers {
            if header.name.to_lowercase() == lowercase_name {
                return true;
            }
        }

        false
    }

    pub fn header_values(&self, name: &str) -> Vec<&str> {
        let mut values = Vec::new();
        let lowercase_name = name.to_lowercase();

        for header in &self.headers {
            if header.name.to_lowercase() == lowercase_name {
                values.push(header.value.as_str());
            }
        }

        values
    }

    pub fn path_and_query(&self) -> String {
        self.path_and_query_skipped.path_and_query_matching.clone()
    }

    pub fn build_sorted_query(query: &str) -> Option<String> {
        let hash_query: BTreeMap<_, _> = parse_query(query.as_bytes()).into_owned().collect();

        let mut query_string = "".to_string();

        for (key, value) in &hash_query {
            query_string.push_str(&utf8_percent_encode(key, QUERY_ENCODE_SET).to_string());

            if !value.is_empty() {
                query_string.push_str("=");
                query_string.push_str(&utf8_percent_encode(value, QUERY_ENCODE_SET).to_string());
            }

            query_string.push_str("&");
        }

        query_string.pop();

        if query_string.is_empty() {
            return None;
        }

        Some(query_string)
    }
}
