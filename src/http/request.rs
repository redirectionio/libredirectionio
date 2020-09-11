use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde::{Deserialize, Serialize};
use super::query::PathAndQueryWithSkipped;
use super::header::Header;

const SIMPLE_ENCODE_SET: &AsciiSet = &CONTROLS;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    pub path_and_query: PathAndQueryWithSkipped,
    pub host: Option<String>,
    pub scheme: Option<String>,
    pub method: Option<String>,
    pub headers: Vec<Header>,
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
