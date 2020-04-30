use crate::ffi_helpers::{c_char_to_str, string_to_c_char};
use crate::api::{RulesMessage, Log};
use std::os::raw::{c_char, c_ulong, c_ushort};
use serde_json::{from_str as json_decoded, to_string as json_encode};
use crate::http::Request;
use crate::http::ffi::{HeaderMap, header_map_to_http_headers};
use crate::action::Action;
use std::ptr::null;

#[no_mangle]
pub unsafe extern fn redirectionio_api_create_rules_message_from_json(content: *mut c_char) -> *const RulesMessage {
    let message_string = match c_char_to_str(content) {
        None => return null() as *const RulesMessage,
        Some(str) => str,
    };

    match json_decoded(message_string) {
        Err(error) => {
            error!("{}", error);
            null() as *const RulesMessage
        },
        Ok(message) => {
            Box::into_raw(Box::new(message))
        }
    }
}

#[no_mangle]
pub unsafe extern fn redirectionio_api_create_log_in_json(_request: *mut Request, code: c_ushort, _response_headers: *const HeaderMap, _action: *mut Action, _proxy: *const c_char, time: c_ulong) -> *const c_char {
    if _action.is_null() || _request.is_null() {
        return null();
    }

    let proxy = c_char_to_str(_proxy).unwrap_or("");
    let action = &*_action;
    let request = &*_request;
    let response_headers = header_map_to_http_headers(_response_headers);

    let log = Log::from_proxy(
        request,
        code,
        response_headers,
        action,
        proxy,
        time
    );

    let log_serialized = match json_encode(&log) {
        Err(_) => return null(),
        Ok(s) => s,
    };

    string_to_c_char(log_serialized)
}
