use crate::action::Action;
use crate::api::{Impact, ImpactResultItem, Log, RouterTrace, RulesMessage};
use crate::ffi_helpers::{c_char_to_str, string_to_c_char};
use crate::http::ffi::{header_map_to_http_headers, HeaderMap};
use crate::http::Request;
use serde_json::{from_str as json_decode, to_string as json_encode};
use std::os::raw::{c_char, c_ushort};
use std::ptr::null;

#[no_mangle]
pub unsafe extern "C" fn redirectionio_api_get_rule_api_version() -> *const c_char {
    string_to_c_char("2.0.0".to_string())
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_api_create_rules_message_from_json(content: *mut c_char) -> *const RulesMessage {
    let message_string = match c_char_to_str(content) {
        None => return null() as *const RulesMessage,
        Some(str) => str,
    };

    match json_decode(message_string) {
        Err(error) => {
            error!("{}", error);
            null() as *const RulesMessage
        }
        Ok(message) => Box::into_raw(Box::new(message)),
    }
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

    let log = Log::from_proxy(request, code, &response_headers, action, proxy, time, client_ip);

    let log_serialized = match json_encode(&log) {
        Err(_) => return null(),
        Ok(s) => s,
    };

    string_to_c_char(log_serialized)
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_api_trace_serialize_and_drop(_trace: *mut RouterTrace) -> *const c_char {
    if _trace.is_null() {
        return null();
    }

    let trace = Box::from_raw(_trace);
    let trace_serialized = match json_encode(&trace) {
        Err(_) => return null(),
        Ok(s) => s,
    };

    string_to_c_char(trace_serialized)
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_api_impact_deserialize(content: *mut c_char) -> *const Impact {
    let impact_string = match c_char_to_str(content) {
        None => return null() as *const Impact,
        Some(str) => str,
    };

    match json_decode(impact_string) {
        Err(error) => {
            error!("{}", error);
            null() as *const Impact
        }
        Ok(impact) => Box::into_raw(Box::new(impact)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_api_impact_result_serialize_and_drop(_result: *mut Vec<ImpactResultItem>) -> *const c_char {
    if _result.is_null() {
        return null();
    }

    let result = Box::from_raw(_result);
    let result_serialized = match json_encode(&result) {
        Err(_) => return null(),
        Ok(s) => s,
    };

    string_to_c_char(result_serialized)
}
