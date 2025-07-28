use std::{os::raw::c_char, ptr::null};

use serde_json::{from_str as json_decode, to_string as json_encode};
use trusted_proxies::{Config, Trusted};

use crate::{
    ffi_helpers::{c_char_to_str, string_to_c_char},
    http::{Addr, Header, PathAndQueryWithSkipped, Request},
    router_config::RouterConfig,
};

#[repr(C)]
#[derive(Debug)]
pub struct HeaderMap {
    name: *const c_char,
    value: *const c_char,
    next: *mut HeaderMap,
}

#[repr(C)]
pub struct TrustedProxies(*mut ());

pub fn http_headers_to_header_map(headers: Vec<Header>) -> *const HeaderMap {
    let mut current: *const HeaderMap = null();

    for header in &headers {
        current = Box::into_raw(Box::new(HeaderMap {
            name: string_to_c_char(header.name.clone()),
            value: string_to_c_char(header.value.clone()),
            next: current as *mut HeaderMap,
        }));
    }

    current
}

pub fn header_map_to_http_headers(header_map: *const HeaderMap) -> Vec<Header> {
    let mut headers = Vec::new();
    let mut current = header_map;

    while !current.is_null() {
        // Safety: current is a valid pointer to a HeaderMap
        let header = unsafe { &*current };
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

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_request_json_deserialize(str: *mut c_char) -> *const Request {
    let request_str = match c_char_to_str(str) {
        None => return null(),
        Some(str) => str,
    };

    let request = match json_decode(request_str) {
        Err(err) => {
            log::error!("cannot deserialize request {err} for string {request_str}");

            return null();
        }
        Ok(request) => request,
    };

    Box::into_raw(Box::new(request))
}

#[unsafe(no_mangle)]
/// # Safety
/// This function must be called with a valid pointer to Request or null pointer
pub unsafe extern "C" fn redirectionio_request_json_serialize(_request: *const Request) -> *const c_char {
    if _request.is_null() {
        return null();
    }

    let request = unsafe { &*_request };
    let request_serialized = match json_encode(request) {
        Err(_) => return null(),
        Ok(request_serialized) => request_serialized,
    };

    string_to_c_char(request_serialized)
}

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_request_create(
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

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_trusted_proxies_create(_proxies_str: *const c_char) -> *const TrustedProxies {
    let mut trusted_proxies = Config::default();

    if let Some(proxies_str) = c_char_to_str(_proxies_str) {
        for proxy in proxies_str.split(',') {
            let proxy_norm = proxy.trim().to_string();

            if !proxy_norm.is_empty() {
                if let Err(e) = trusted_proxies.add_trusted_ip(proxy_norm.as_str()) {
                    log::warn!("cannot parse trusted proxy {proxy_norm}: {e}");
                }
            }
        }
    }

    Box::into_raw(Box::new(TrustedProxies(Box::into_raw(Box::new(trusted_proxies)) as *mut ())))
}

#[unsafe(no_mangle)]
/// # Safety
/// This function must be called with a valid pointer to TrustedProxies or null pointer
pub unsafe extern "C" fn redirectionio_trusted_proxies_add_proxy(_trusted_proxies: *mut TrustedProxies, _proxy_str: *const c_char) {
    if _trusted_proxies.is_null() {
        return;
    }

    let proxy_str = match c_char_to_str(_proxy_str).map(|str| str.to_string()) {
        None => return,
        Some(s) => s,
    };

    // Safety: _trusted_proxies is a valid pointer to a TrustedProxies
    let trusted_proxies = unsafe { &mut *_trusted_proxies };
    // Safety: trusted_proxies.0 is a valid pointer to a Config
    // It should be created once and never be freed, so it's safe to dereference it as it will never be freed
    let config = unsafe { &mut *(trusted_proxies.0 as *mut Config) };

    if let Err(e) = config.add_trusted_ip(proxy_str.as_str()) {
        log::warn!("cannot parse trusted proxy {proxy_str}: {e}");
    }
}

#[unsafe(no_mangle)]
/// # Safety
///
/// This function must be called with a valid pointer to Request or null pointer
/// and a valid pointer to TrustedProxies or null pointer
pub unsafe extern "C" fn redirectionio_request_set_remote_addr(
    _request: *mut Request,
    _remote_addr_str: *const c_char,
    _trusted_proxies: *const TrustedProxies,
) {
    if _request.is_null() {
        return;
    }

    // Safety: _request is a valid pointer to a Request
    let request = unsafe { &mut *_request };

    let remote_addr_str = match c_char_to_str(_remote_addr_str).map(|str| str.to_string()) {
        None => return,
        Some(s) => s,
    };

    let remote_addr = match remote_addr_str.parse::<Addr>() {
        Err(_) => {
            return;
        }
        Ok(addr) => addr,
    };

    let config = if _trusted_proxies.is_null() {
        &Config::default()
    } else {
        // Safety: _trusted_proxies is a valid pointer to a TrustedProxies
        let trusted_proxies = unsafe { &*_trusted_proxies };

        // SAFETY: trusted_proxies.0 is a valid pointer to a Config
        // It should be created once and never be freed, so it's safe to dereference it as it will never be freed
        unsafe { &*(trusted_proxies.0 as *mut Config) }
    };

    let trusted = Trusted::from(remote_addr.addr, request, config);

    request.set_remote_ip(trusted.ip());
}

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_request_from_str(_url: *const c_char) -> *const Request {
    let url = c_char_to_str(_url).unwrap_or("/");

    match url.parse::<Request>() {
        Err(err) => {
            log::error!("cannot create request for url {url}: {err}");

            null()
        }
        Ok(request) => Box::into_raw(Box::new(request)),
    }
}

#[unsafe(no_mangle)]
/// # Safety
///
/// This function must be called with a valid pointer to Request or null pointer
pub unsafe extern "C" fn redirectionio_request_drop(_request: *mut Request) {
    if _request.is_null() {
        return;
    }

    // Safety: _request is a valid pointer to a Request
    drop(unsafe { Box::from_raw(_request) });
}
