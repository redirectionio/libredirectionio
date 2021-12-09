use crate::ffi_helpers::{c_char_to_str, string_to_c_char};
use crate::http::{Header, PathAndQueryWithSkipped, Request, TrustedProxies};
use crate::router::RouterConfig;
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
        current = header.next;

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
    let host = c_char_to_str(_host).map(|str| str.to_string());
    let scheme = c_char_to_str(_scheme).map(|str| str.to_string());
    let method = c_char_to_str(_method).map(|str| str.to_string());

    let config = RouterConfig::default();
    let mut request = Request::new(
        PathAndQueryWithSkipped::from_config(&config, uri),
        uri.to_string(),
        host,
        scheme,
        method,
        None,
        None,
    );
    let headers = header_map_to_http_headers(header_map);

    for header in headers {
        request.add_header(header.name, header.value, config.ignore_header_case);
    }

    Box::into_raw(Box::new(request))
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn redirectionio_trusted_proxies_create(_proxies_str: *const c_char) -> *const TrustedProxies {
    let mut trusted_proxies = TrustedProxies::default();

    if let Some(proxies_str) = c_char_to_str(_proxies_str) {
        for proxy in proxies_str.split(',') {
            let proxy_norm = proxy.trim().to_string();

            if !proxy_norm.is_empty() {
                if let Err(e) = trusted_proxies.add_trusted_proxy(proxy_norm.as_str()) {
                    log::warn!("cannot parse trusted proxy {}: {}", proxy_norm, e);
                }
            }
        }
    }

    Box::into_raw(Box::new(trusted_proxies))
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn redirectionio_trusted_proxies_add_proxy(_trusted_proxies: *mut TrustedProxies, _proxy_str: *const c_char) {
    if _trusted_proxies.is_null() {
        return;
    }

    let proxy_str = match c_char_to_str(_proxy_str).map(|str| str.to_string()) {
        None => return,
        Some(s) => s,
    };

    let trusted_proxies = &mut *_trusted_proxies;

    if let Err(e) = trusted_proxies.add_trusted_proxy(proxy_str.as_str()) {
        log::warn!("cannot parse trusted proxy {}: {}", proxy_str, e);
    }
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn redirectionio_request_set_remote_addr(
    _request: *mut Request,
    _remote_addr_str: *const c_char,
    _trusted_proxies: *const TrustedProxies,
) {
    if _request.is_null() {
        return;
    }

    let request = &mut *_request;

    let remote_addr_str = match c_char_to_str(_remote_addr_str).map(|str| str.to_string()) {
        None => return,
        Some(s) => s,
    };

    if _trusted_proxies.is_null() {
        let empty_proxy = TrustedProxies::default();

        request.set_remote_ip(remote_addr_str, &empty_proxy);
    } else {
        let trusted_proxies = &*_trusted_proxies;

        request.set_remote_ip(remote_addr_str, trusted_proxies);
    }
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
