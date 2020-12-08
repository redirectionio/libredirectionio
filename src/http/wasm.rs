use super::STATIC_QUERY_PARAM_SKIP_BUILDER;
use super::{Header, Request as RedirectionioRequest};
use serde_json::to_string as json_encode;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Request {
    #[wasm_bindgen(skip)]
    pub request: RedirectionioRequest,
}

#[wasm_bindgen]
pub struct HeaderMap {
    #[wasm_bindgen(skip)]
    pub headers: Vec<Header>,
}

#[wasm_bindgen]
impl Request {
    #[wasm_bindgen(constructor)]
    pub fn new(uri: String, host: String, scheme: String, method: String) -> Request {
        Request {
            request: RedirectionioRequest {
                headers: Vec::new(),
                host: Some(host),
                method: Some(method),
                scheme: Some(scheme),
                path_and_query: STATIC_QUERY_PARAM_SKIP_BUILDER.build_query_param_skipped(uri.as_str()),
            },
        }
    }

    pub fn add_header(&mut self, name: String, value: String) {
        self.request.add_header(name, value)
    }

    pub fn serialize(&self) -> String {
        match json_encode(&self.request) {
            Err(_) => "".to_string(),
            Ok(request_serialized) => request_serialized,
        }
    }
}

#[wasm_bindgen]
impl HeaderMap {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> HeaderMap {
        HeaderMap { headers: Vec::new() }
    }

    pub fn add_header(&mut self, name: String, value: String) {
        self.headers.push(Header { name, value })
    }

    pub fn len(&self) -> usize {
        self.headers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.headers.is_empty()
    }

    pub fn get_header_name(&self, index: usize) -> String {
        match self.headers.get(index) {
            None => "".to_string(),
            Some(header) => header.name.clone(),
        }
    }

    pub fn get_header_value(&self, index: usize) -> String {
        match self.headers.get(index) {
            None => "".to_string(),
            Some(header) => header.value.clone(),
        }
    }
}