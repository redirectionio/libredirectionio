use std::{
    os::raw::{c_char, c_ushort},
    ptr::null,
};

use serde_json::to_string as json_encode;

use crate::{
    action::Action,
    api::Log,
    ffi_helpers::{c_char_to_str, string_to_c_char},
    http::{
        Request,
        ffi::{HeaderMap, header_map_to_http_headers},
    },
};

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_api_get_rule_api_version() -> *const c_char {
    string_to_c_char("2.0.0".to_string())
}

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_api_create_log_in_json(
    _request: *mut Request,
    code: c_ushort,
    _response_headers: *const HeaderMap,
    _action: *mut Action,
    _proxy: *const c_char,
    time: u64,
    action_match_time: u64,
    proxy_response_time: u64,
    _client_ip: *const c_char,
) -> *const c_char {
    if _request.is_null() {
        return null();
    }

    let proxy = c_char_to_str(_proxy).unwrap_or("");
    let client_ip = c_char_to_str(_client_ip).unwrap_or("");
    // Safety: _action is a valid pointer to a Action
    let action = if _action.is_null() { None } else { Some(unsafe { &*_action }) };
    // Safety: _request is a valid pointer to a Request
    let request = unsafe { &*_request };
    let response_headers = header_map_to_http_headers(_response_headers);

    let log = Log::from_proxy(
        request,
        code,
        &response_headers,
        action,
        proxy,
        time as u128,
        action_match_time as u128,
        if proxy_response_time == 0 {
            None
        } else {
            Some(proxy_response_time as u128)
        },
        client_ip,
    );

    let log_serialized = match json_encode(&log) {
        Err(_) => return null(),
        Ok(s) => s,
    };

    string_to_c_char(log_serialized)
}
