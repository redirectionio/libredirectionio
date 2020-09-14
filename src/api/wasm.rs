use super::Log;
use crate::action::wasm::Action;
use crate::http::wasm::{HeaderMap, Request};
use serde_json::to_string as json_encode;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn create_log_in_json(
    request: Request,
    status_code: u16,
    response_headers: HeaderMap,
    action: &Action,
    proxy: String,
    time: u64,
) -> String {
    let log = Log::from_proxy(
        &request.request,
        status_code,
        &response_headers.headers,
        action.action.as_ref(),
        proxy.as_str(),
        time,
    );

    match json_encode(&log) {
        Err(_) => "".to_string(),
        Ok(s) => s,
    }
}
