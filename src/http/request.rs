use serde::{Deserialize, Serialize};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use crate::http::header::Header;

const SIMPLE_ENCODE_SET: &AsciiSet = &CONTROLS;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    url: String,
    method: Option<String>,
    headers: Vec<Header>,
}

impl Request {
    pub fn new(url: String, method: Option<String>) -> Request {
        Request {
            url,
            method,
            headers: Vec::new(),
        }
    }

    pub fn add_header(&mut self, name: String, value: String) {
        self.headers.push(Header {
            name,
            value,
        });
    }

    pub fn to_http_request(&self) -> http::Result<http::Request<()>> {
        let mut builder = http::Request::<()>::builder();

        let url = utf8_percent_encode(self.url.replace(" ", "%20").as_str(), SIMPLE_ENCODE_SET).to_string();

        builder = builder.uri(url.as_str());

        if let Some(method) = self.method.as_ref() {
            builder = builder.method(method.as_str());
        }

        for header in &self.headers {
            builder = builder.header(header.name.as_str(), header.value.clone());
        }

        builder.body(())
    }
}
