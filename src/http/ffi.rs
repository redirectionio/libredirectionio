use super::STATIC_QUERY_PARAM_SKIP_BUILDER;
use crate::ffi_helpers::{c_char_to_str, string_to_c_char};
use crate::http::{Header, Request};
use serde_json::{from_str as json_decode, to_string as json_encode};
use std::os::raw::c_char;
use std::ptr::null;

#[repr(C)]
#[derive(Debug)]
pub struct HeaderMap {
    name: *const c_char,
    value: *const c_char,
    next: *mut HeaderMap,
}

/// # Safety
pub unsafe fn http_headers_to_header_map(headers: Vec<Header>) -> *const HeaderMap {
    let mut current: *const HeaderMap = null() as *const HeaderMap;

    for header in &headers {
        current = Box::into_raw(Box::new(HeaderMap {
            name: string_to_c_char(header.name.clone()),
            value: string_to_c_char(header.value.clone()),
            next: current as *mut HeaderMap,
        }));
    }

    current
}

/// # Safety
pub unsafe fn header_map_to_http_headers(header_map: *const HeaderMap) -> Vec<Header> {
    let mut headers = Vec::new();
    let mut current = header_map;

    while !current.is_null() {
        let header = &*current;
        let name = match c_char_to_str(header.name) {
            None => continue,
            Some(s) => s,
        };
        let value = match c_char_to_str(header.value) {
            None => continue,
            Some(s) => s,
        };

        headers.push(Header {
            name: name.to_string(),
            value: value.to_string(),
        });

        current = header.next;
    }

    headers
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn redirectionio_request_json_deserialize(str: *mut c_char) -> *const Request {
    let request_str = match c_char_to_str(str) {
        None => return null() as *const Request,
        Some(str) => str,
    };

    let request = match json_decode(request_str) {
        Err(err) => {
            error!("cannot deserialize request {} for string {}", err, request_str);

            return null() as *const Request;
        }
        Ok(request) => request,
    };

    Box::into_raw(Box::new(request))
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn redirectionio_request_json_serialize(_request: *const Request) -> *const c_char {
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
/// # Safety
pub unsafe extern "C" fn redirectionio_request_create(
    _uri: *const c_char,
    _host: *const c_char,
    _scheme: *const c_char,
    _method: *const c_char,
    header_map: *const HeaderMap,
) -> *const Request {
    let uri = c_char_to_str(_uri).unwrap_or("/");
    let host = match c_char_to_str(_host) {
        None => None,
        Some(str) => Some(str.to_string()),
    };
    let scheme = match c_char_to_str(_scheme) {
        None => None,
        Some(str) => Some(str.to_string()),
    };
    let method = match c_char_to_str(_method) {
        None => None,
        Some(str) => Some(str.to_string()),
    };

    let mut request = Request::new(STATIC_QUERY_PARAM_SKIP_BUILDER.build_query_param_skipped(uri), host, scheme, method);
    let headers = header_map_to_http_headers(header_map);

    for header in headers {
        request.add_header(header.name, header.value);
    }

    Box::into_raw(Box::new(request))
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn redirectionio_request_from_str(_url: *const c_char) -> *const Request {
    let url = c_char_to_str(_url).unwrap_or("/");

    match url.parse::<Request>() {
        Err(err) => {
            error!("cannot create request for url {}: {}", url, err);

            null() as *const Request
        }
        Ok(request) => Box::into_raw(Box::new(request)),
    }
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn redirectionio_request_drop(_request: *mut Request) {
    if _request.is_null() {
        return;
    }

    Box::from_raw(_request);
}
