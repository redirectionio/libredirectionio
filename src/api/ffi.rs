use crate::action::Action;
use crate::api::Log;
use crate::ffi_helpers::{c_char_to_str, string_to_c_char};
use crate::http::ffi::{header_map_to_http_headers, HeaderMap};
use crate::http::Request;
use serde_json::to_string as json_encode;
use std::os::raw::{c_char, c_ushort};
use std::ptr::null;

#[no_mangle]
pub unsafe extern "C" fn redirectionio_api_get_rule_api_version() -> *const c_char {
    string_to_c_char("2.0.0".to_string())
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_api_create_log_in_json(
    _request: *mut Request,
    code: c_ushort,
    _response_headers: *const HeaderMap,
    _action: *mut Action,
    _proxy: *const c_char,
    time: u64,
    _client_ip: *const c_char,
) -> *const c_char {
    if _request.is_null() {
        return null();
    }

    let proxy = c_char_to_str(_proxy).unwrap_or("");
    let client_ip = c_char_to_str(_client_ip).unwrap_or("");
    let action = if _action.is_null() { None } else { Some(&*_action) };
    let request = &*_request;
    let response_headers = header_map_to_http_headers(_response_headers);

    let log = Log::from_proxy(request, code, &response_headers, action, proxy, time as u128, client_ip, None);

    let log_serialized = match json_encode(&log) {
        Err(_) => return null(),
        Ok(s) => s,
    };

    string_to_c_char(log_serialized)
}
