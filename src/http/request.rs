use super::header::Header;
use super::query::PathAndQueryWithSkipped;
use super::STATIC_QUERY_PARAM_SKIP_BUILDER;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

const SIMPLE_ENCODE_SET: &AsciiSet = &CONTROLS;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    pub path_and_query: PathAndQueryWithSkipped,
    pub host: Option<String>,
    pub scheme: Option<String>,
    pub method: Option<String>,
    pub headers: Vec<Header>,
}

impl FromStr for Request {
    type Err = http::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let http_request = http::Request::<()>::builder().uri(s).method("GET").body(())?;

        Ok(Request::new(
            STATIC_QUERY_PARAM_SKIP_BUILDER.build_query_param_skipped(match http_request.uri().path_and_query() {
                None => "",
                Some(path_and_query) => path_and_query.as_str(),
            }),
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
    pub fn new(path_and_query: PathAndQueryWithSkipped, host: Option<String>, scheme: Option<String>, method: Option<String>) -> Request {
        Request {
            path_and_query,
            host,
            scheme,
            method,
            headers: Vec::new(),
        }
    }

    pub fn add_header(&mut self, name: String, value: String) {
        self.headers.push(Header { name, value });
    }

    pub fn to_http_request(&self) -> http::Result<http::Request<()>> {
        let mut request_builder = http::Request::<()>::builder();
        let mut uri_builder = http::Uri::builder();

        let url = utf8_percent_encode(self.path_and_query.path_and_query.replace(" ", "%20").as_str(), SIMPLE_ENCODE_SET).to_string();

        uri_builder = uri_builder.path_and_query(url.as_str());

        if let Some(host) = self.host.as_ref() {
            uri_builder = uri_builder.authority(host.as_str());
        }

        if let Some(scheme) = self.scheme.as_ref() {
            uri_builder = uri_builder.scheme(scheme.as_str());
        }

        request_builder = request_builder.uri(uri_builder.build()?);

        if let Some(method) = self.method.as_ref() {
            request_builder = request_builder.method(method.as_str());
        }

        for header in &self.headers {
            request_builder = request_builder.header(header.name.as_str(), header.value.clone());
        }

        request_builder.body(())
    }
}
