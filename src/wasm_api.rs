use crate::action::Action as RedirectionioAction;
use crate::api::Log;
use crate::filter::FilterBodyAction;
use crate::http::{Header, PathAndQueryWithSkipped, Request as RedirectionioRequest, TrustedProxies};
use crate::RouterConfig;
use chrono::Utc;
use serde_json::{from_str as json_decode, to_string as json_encode};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use wasm_bindgen::prelude::*;

#[wasm_bindgen()]
pub struct Request {
    #[wasm_bindgen(skip)]
    pub request: RedirectionioRequest,
}

#[wasm_bindgen()]
pub struct HeaderMap {
    #[wasm_bindgen(skip)]
    pub headers: Vec<Header>,
}

#[wasm_bindgen()]
pub struct Action {
    #[wasm_bindgen(skip)]
    pub action: Option<RedirectionioAction>,
}

#[wasm_bindgen()]
pub struct BodyFilter {
    #[wasm_bindgen(skip)]
    pub filter: Option<FilterBodyAction>,
}

#[wasm_bindgen()]
impl Request {
    #[wasm_bindgen(constructor)]
    pub fn new(uri: String, host: String, scheme: String, method: String) -> Request {
        let config = RouterConfig::default();

        Request {
            request: RedirectionioRequest {
                headers: Vec::new(),
                host: Some(host),
                method: Some(method),
                scheme: Some(scheme),
                path_and_query_skipped: PathAndQueryWithSkipped::from_config(&config, uri.as_str()),
                path_and_query: Some(uri),
                remote_addr: None,
                created_at: Some(Utc::now()),
                sampling_override: None,
            },
        }
    }

    pub fn set_remote_ip(&mut self, remote_addr_str: String) {
        self.request.set_remote_ip(remote_addr_str, &TrustedProxies::default());
    }

    pub fn add_header(&mut self, name: String, value: String) {
        self.request.add_header(name, value, false)
    }

    pub fn serialize(&self) -> String {
        match json_encode(&self.request) {
            Err(_) => "".to_string(),
            Ok(request_serialized) => request_serialized,
        }
    }

    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.request.hash(&mut hasher);

        hasher.finish()
    }
}

#[wasm_bindgen()]
impl HeaderMap {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> HeaderMap {
        HeaderMap { headers: Vec::new() }
    }

    pub fn add_header(&mut self, name: String, value: String) {
        self.headers.push(Header { name, value })
    }

    pub fn remove_header(&mut self, name: String) {
        self.headers.retain(|header| header.name != name)
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

#[wasm_bindgen()]
impl Action {
    #[wasm_bindgen(constructor)]
    pub fn new(action_serialized: String) -> Action {
        let action = match json_decode(action_serialized.as_str()) {
            Err(error) => {
                log::error!("Unable to deserialize \"{}\" to action: {}", action_serialized, error,);

                None
            }
            Ok(action) => Some(action),
        };

        Action { action }
    }

    pub fn empty() -> Action {
        Action { action: None }
    }

    pub fn get_status_code(&mut self, response_status_code: u16) -> u16 {
        if let Some(action) = self.action.as_mut() {
            return action.get_status_code(response_status_code, None);
        }

        0
    }

    pub fn filter_headers(&mut self, headers: HeaderMap, response_status_code: u16, add_rule_ids_header: bool) -> HeaderMap {
        if self.action.is_none() {
            return headers;
        }

        let action = self.action.as_mut().unwrap();
        let new_headers = action.filter_headers(headers.headers, response_status_code, add_rule_ids_header, None);

        HeaderMap { headers: new_headers }
    }

    pub fn create_body_filter(&mut self, response_status_code: u16, headers: &HeaderMap) -> BodyFilter {
        if self.action.is_none() {
            return BodyFilter { filter: None };
        }

        let action = self.action.as_mut().unwrap();
        let filter = action.create_filter_body(response_status_code, &headers.headers);

        BodyFilter { filter }
    }

    pub fn should_log_request(&mut self, response_status_code: u16) -> bool {
        if self.action.is_none() {
            return true;
        }

        let action = self.action.as_mut().unwrap();

        action.should_log_request(true, response_status_code, None)
    }
}

#[wasm_bindgen()]
impl BodyFilter {
    pub fn is_null(&self) -> bool {
        self.filter.is_none()
    }

    pub fn filter(&mut self, data: Vec<u8>) -> Vec<u8> {
        match self.filter.as_mut() {
            None => data,
            Some(filter) => filter.filter(data, None),
        }
    }

    pub fn end(&mut self) -> Vec<u8> {
        match self.filter.as_mut() {
            None => Vec::new(),
            Some(filter) => filter.end(None),
        }
    }
}

#[wasm_bindgen()]
pub fn init_log() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Error));
}

#[wasm_bindgen()]
pub fn create_log_in_json(
    request: Request,
    status_code: u16,
    response_headers: HeaderMap,
    action: &Action,
    proxy: String,
    time: u64,
    client_ip: String,
) -> String {
    let log = Log::from_proxy(
        &request.request,
        status_code,
        &response_headers.headers,
        action.action.as_ref(),
        proxy.as_str(),
        time.into(),
        client_ip.as_str(),
        None,
    );

    match json_encode(&log) {
        Err(_) => "".to_string(),
        Ok(s) => s,
    }
}
