use std::os::raw::c_char;
use std::ptr::null;
use serde_json::{from_str as json_decode, to_string as json_encode};
use crate::http::{Request, Header};
use crate::ffi_helpers::{c_char_to_str, string_to_c_char};

#[no_mangle]
pub unsafe extern fn redirectionio_request_json_deserialize(str: *mut c_char) -> Option<*mut Request> {
    let request_str = c_char_to_str(str)?;

    let request = match json_decode(request_str) {
        Err(_) => return None,
        Ok(request) => request,
    };

    Some(Box::into_raw(Box::new(request)))
}

#[no_mangle]
pub unsafe extern fn redirectionio_request_json_serialize(_request: *mut Request) -> *const c_char {
    if _request.is_null() {
        return null();
    }

    let request = &*_request;
    let request_serialized = match json_encode(request) {
        Err(_) => return null(),
        Ok(request_serialized) => request_serialized,
    };

    string_to_c_char(request_serialized)
}

#[no_mangle]
pub unsafe extern fn redirectionio_request_create(_uri: *const c_char, _method: *const c_char) -> Option<*mut Request> {
    let uri = c_char_to_str(_uri)?;
    let method = match c_char_to_str(_method) {
        None => None,
        Some(str) => Some(str.to_string()),
    };

    let request = Request::new(uri.to_string(), method);

    Some(Box::into_raw(Box::new(request)))
}

#[no_mangle]
pub unsafe extern fn redirectionio_request_add_header(_request: *mut Request, _key: *const c_char, _value: *const c_char) {
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
pub unsafe extern fn redirectionio_header_map_create() -> *mut Vec<Header> {
    let request_header = Vec::new();

    Box::into_raw(Box::new(request_header))
}

#[no_mangle]
pub unsafe extern fn redirectionio_header_map_add_header(_headers: *mut Vec<Header>, _key: *const c_char, _value: *const c_char) {
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

    headers.push(Header {
        name: key,
        value,
    })
}
