use std::{collections::BTreeMap, net::IpAddr, str::FromStr};

use chrono::{DateTime, Utc};
#[cfg(feature = "router")]
use http::Error;
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use serde::{Deserialize, Serialize};
use trusted_proxies::RequestInformation;
use url::form_urlencoded::parse as parse_query;

use super::{header::Header, query::PathAndQueryWithSkipped};
#[cfg(feature = "router")]
use crate::api::Example;
#[cfg(feature = "router")]
use crate::http::sanitize_url;
#[cfg(feature = "router")]
use crate::router_config::RouterConfig;

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
    pub remote_addr: Option<IpAddr>,
    pub created_at: Option<DateTime<Utc>>,
    pub sampling_override: Option<bool>,
}

impl FromStr for Request {
    type Err = http::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let http_request = http::Request::<()>::builder().uri(s).method("GET").body(())?;
        let path_and_query_str = match http_request.uri().path_and_query() {
            None => "",
            Some(path_and_query) => path_and_query.as_str(),
        };

        Ok(Request::new(
            PathAndQueryWithSkipped::from_static(path_and_query_str),
            path_and_query_str.to_string(),
            http_request.uri().authority().map(|s| s.to_string()),
            http_request.uri().scheme_str().map(|s| s.to_string()),
            None,
            None,
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
        remote_addr: Option<IpAddr>,
        sampling_override: Option<bool>,
    ) -> Request {
        Request {
            path_and_query_skipped,
            path_and_query: Some(path_and_query),
            host,
            scheme,
            method,
            headers: Vec::new(),
            remote_addr,
            created_at: Some(Utc::now()),
            sampling_override,
        }
    }

    #[cfg(feature = "router")]
    pub fn from_config(
        config: &RouterConfig,
        path_and_query: String,
        host: Option<String>,
        scheme: Option<String>,
        method: Option<String>,
        remote_addr: Option<IpAddr>,
        sampling_override: Option<bool>,
    ) -> Request {
        Request {
            path_and_query_skipped: PathAndQueryWithSkipped::from_config(config, path_and_query.as_str()),
            path_and_query: Some(path_and_query),
            host: host.map(|s| if config.ignore_host_case { s.to_lowercase() } else { s }),
            scheme,
            method,
            remote_addr,
            headers: Vec::new(),
            created_at: Some(Utc::now()),
            sampling_override,
        }
    }

    #[cfg(feature = "router")]
    pub fn from_example(router_config: &RouterConfig, example: &Example) -> Result<Self, Error> {
        let method = example.method.as_deref().unwrap_or("GET");
        let url = sanitize_url(example.url.as_str());
        let http_request = http::Request::<()>::builder().uri(url.as_str()).method(method).body(())?;
        let path_and_query_str = match http_request.uri().path_and_query() {
            None => "",
            Some(path_and_query) => path_and_query.as_str(),
        };

        let mut request = Request::from_config(
            router_config,
            path_and_query_str.to_string(),
            http_request.uri().authority().map(|s| s.to_string()),
            http_request.uri().scheme_str().map(|s| s.to_string()),
            example.method.clone(),
            None,
            None,
        );

        if let Some(headers) = &example.headers {
            for header in headers {
                request.add_header(header.name.clone(), header.value.clone(), false);
            }
        }

        if let Some(ip) = &example.ip_address {
            request.remote_addr = Some(IpAddr::from_str(ip).unwrap());
        }

        if let Some(datetime) = &example.datetime {
            request.set_created_at(Some(datetime.to_string()));
        }

        Ok(request)
    }

    #[cfg(feature = "router")]
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
            remote_addr: request.remote_addr,
            created_at: request.created_at,
            sampling_override: request.sampling_override,
        }
    }

    pub fn add_header(&mut self, name: String, value: String, ignore_case: bool) {
        self.headers.push(Header {
            name,
            value: if ignore_case { value.to_lowercase() } else { value },
        });
    }

    pub fn set_created_at(&mut self, created_at: Option<String>) {
        match created_at {
            None => (),
            Some(created_at) => match created_at.parse::<DateTime<Utc>>() {
                Ok(dt) => self.created_at = Some(dt),
                Err(err) => {
                    log::error!("cannot parse datetime {}: {}", created_at, err);
                }
            },
        };
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

    pub fn set_remote_ip(&mut self, remote_ip: IpAddr) {
        self.remote_addr = Some(remote_ip);
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

    pub fn header_value(&self, name: &str) -> Option<String> {
        let values = self.header_values(name);

        if values.is_empty() { None } else { Some(values.join(",")) }
    }

    pub fn path_and_query(&self) -> String {
        match &self.path_and_query_skipped.path_and_query_matching {
            None => self.path_and_query_skipped.path_and_query.clone(),
            Some(path) => path.clone(),
        }
    }

    pub fn build_sorted_query(query: &str) -> Option<String> {
        let hash_query: BTreeMap<_, _> = parse_query(query.as_bytes()).into_owned().collect();

        let mut query_string = "".to_string();

        for (key, value) in &hash_query {
            query_string.push_str(&utf8_percent_encode(key, QUERY_ENCODE_SET).to_string());

            if !value.is_empty() {
                query_string.push('=');
                query_string.push_str(&utf8_percent_encode(value, QUERY_ENCODE_SET).to_string());
            }

            query_string.push('&');
        }

        query_string.pop();

        if query_string.is_empty() {
            return None;
        }

        Some(query_string)
    }
}

impl RequestInformation for Request {
    fn is_host_header_allowed(&self) -> bool {
        true
    }

    fn host_header(&self) -> Option<&str> {
        self.headers
            .iter()
            .find(|header| header.name.to_lowercase() == "host")
            .map(|header| header.value.as_str())
    }

    fn authority(&self) -> Option<&str> {
        self.host()
    }

    fn forwarded(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.headers
            .iter()
            .filter(|header| header.name.to_lowercase() == "forwarded")
            .map(|header| header.value.as_str())
    }

    fn x_forwarded_for(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.headers
            .iter()
            .filter(|header| header.name.to_lowercase() == "x-forwarded-for")
            .map(|header| header.value.as_str())
    }

    fn x_forwarded_host(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.headers
            .iter()
            .filter(|header| header.name.to_lowercase() == "x-forwarded-host")
            .map(|header| header.value.as_str())
    }

    fn x_forwarded_proto(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.headers
            .iter()
            .filter(|header| header.name.to_lowercase() == "x-forwarded-proto")
            .map(|header| header.value.as_str())
    }

    fn x_forwarded_by(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.headers
            .iter()
            .filter(|header| header.name.to_lowercase() == "x-forwarded-by")
            .map(|header| header.value.as_str())
    }

    fn default_scheme(&self) -> Option<&str> {
        self.scheme()
    }
}
