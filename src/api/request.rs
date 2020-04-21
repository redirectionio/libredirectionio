use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    url: String,
    method: Option<String>,
    headers: Vec<MessageHeader>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageHeader {
    pub name: String,
    pub value: String,
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
        self.headers.push(MessageHeader {
            name,
            value,
        });
    }

    pub fn to_http_request(&self) -> http::Request<()> {
        let mut builder = http::Request::<()>::builder();

        builder = builder.uri(self.url.as_str());

        if let Some(method) = self.method.as_ref() {
            builder = builder.method(method.as_str());
        }

        for header in &self.headers {
            builder = builder.header(header.name.as_str(), header.value.clone());
        }

        builder.body(()).expect("")
    }
}
