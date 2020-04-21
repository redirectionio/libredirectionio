use crate::ffi_helpers::{c_char_to_string, c_char_to_str};
use crate::api::{RulesMessage, Request, MessageHeader};
use std::os::raw::c_char;
use serde_json::from_str;

#[no_mangle]
pub unsafe extern fn redirectionio_create_rules_message_from_json(content: *mut c_char) -> Option<*mut RulesMessage> {
    let message_string = c_char_to_string(content)?;
    let message_result = from_str(message_string.as_str());

    if message_result.is_err() {
        error!("{}", message_result.err().unwrap());

        return None;
    }

    let message: RulesMessage = message_result.unwrap();

    Some(Box::into_raw(Box::new(message)))
}

#[no_mangle]
pub unsafe extern fn redirectionio_create_request(_uri: *const c_char, _method: *const c_char) -> Option<*mut Request> {
    let uri = c_char_to_str(_uri)?;
    let method = match c_char_to_str(_method) {
        None => None,
        Some(str) => Some(str.to_string()),
    };

    let request = Request::new(uri.to_string(), method);

    Some(Box::into_raw(Box::new(request)))
}

#[no_mangle]
pub unsafe extern fn redirectionio_add_request_header(_request: *mut Request, _key: *const c_char, _value: *const c_char) {
    if _request.is_null() {
        return;
    }

    let key = match c_char_to_str(_key) {
        None => return,
        Some(key) => key.to_string(),
    };

    let value = match c_char_to_str(_value) {
        None => return,
        Some(value) => value.to_string(),
    };

    let mut request = Box::from_raw(_request);

    request.add_header(key, value);
}

#[no_mangle]
pub unsafe extern fn redirectionio_action_create_request_header_vec() -> *mut Vec<MessageHeader> {
    let request_header = Vec::new();

    Box::into_raw(Box::new(request_header))
}

#[no_mangle]
pub unsafe extern fn redirectionio_action_add_request_header(_headers: *mut Vec<MessageHeader>, _key: *const c_char, _value: *const c_char) {
    if _headers.is_null() {
        return;
    }

    let key = match c_char_to_str(_key) {
        None => return,
        Some(key) => key.to_string(),
    };

    let value = match c_char_to_str(_value) {
        None => return,
        Some(value) => value.to_string(),
    };

    let headers = &mut *_headers;

    headers.push(MessageHeader {
        name: key,
        value,
    })
}

// @TODO Add serialization / deserialization of rules message in bincode
