use std::{ffi::CString, os::raw::c_char, ptr::null};

use serde_json::{from_str as json_decode, to_string as json_encode};
use trusted_proxies::{Config, RequestInformation, Trusted};

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

/// Free a header map previously returned by `redirectionio_action_header_filter_filter`.
///
/// The returned list, along with each header name and value, is allocated with
/// Rust's allocator, so it must be reclaimed by Rust as well rather than with the
/// C `free()` function.
///
/// # Safety
///
/// This function must be called with a pointer returned by
/// `redirectionio_action_header_filter_filter`, or a null pointer. It must not be
/// called with a header map allocated on the caller side, and the list must be
/// dropped at most once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn redirectionio_header_map_drop(header_map: *const HeaderMap) {
    let mut current = header_map as *mut HeaderMap;

    while !current.is_null() {
        // Safety: current is a valid pointer to a Rust-allocated HeaderMap node
        let node = unsafe { Box::from_raw(current) };

        if !node.name.is_null() {
            // Safety: name was created with CString::into_raw in string_to_c_char
            drop(unsafe { CString::from_raw(node.name as *mut c_char) });
        }

        if !node.value.is_null() {
            // Safety: value was created with CString::into_raw in string_to_c_char
            drop(unsafe { CString::from_raw(node.value as *mut c_char) });
        }

        current = node.next;
    }
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
            tracing::error!("cannot deserialize request {err} for string {request_str}");

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

            if !proxy_norm.is_empty()
                && let Err(e) = trusted_proxies.add_trusted_ip(proxy_norm.as_str())
            {
                tracing::warn!("cannot parse trusted proxy {proxy_norm}: {e}");
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
        tracing::warn!("cannot parse trusted proxy {proxy_str}: {e}");
    }
}

#[unsafe(no_mangle)]
/// # Safety
///
/// This function must be called with a valid pointer to TrustedProxies or null pointer
pub unsafe extern "C" fn redirectionio_trusted_proxies_drop(_trusted_proxies: *mut TrustedProxies) {
    if _trusted_proxies.is_null() {
        return;
    }

    // Safety: _trusted_proxies is a valid pointer to a TrustedProxies
    let trusted_proxies = unsafe { Box::from_raw(_trusted_proxies) };

    // Also free the inner Config that was Box::into_raw'd during creation
    if !trusted_proxies.0.is_null() {
        drop(unsafe { Box::from_raw(trusted_proxies.0 as *mut Config) });
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
    unsafe { redirectionio_request_set_forwarded(_request, _remote_addr_str, _trusted_proxies, 0, 0) }
}

#[unsafe(no_mangle)]
/// # Safety
///
/// This function must be called with a valid pointer to Request or null pointer
/// and a valid pointer to TrustedProxies or null pointer
///
/// Same as `redirectionio_request_set_remote_addr`, but also takes the scheme and the host from a
/// trusted proxy's `Forwarded` header (RFC 7239). `X-Forwarded-Proto` and `X-Forwarded-Host` are
/// not used, they are trivially spoofable.
///
/// Only allow an override for a value you had to guess: an explicit one must win.
pub unsafe extern "C" fn redirectionio_request_set_forwarded(
    _request: *mut Request,
    _remote_addr_str: *const c_char,
    _trusted_proxies: *const TrustedProxies,
    _allow_scheme_override: u8,
    _allow_host_override: u8,
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

    let remote_ip = trusted.ip();

    // `Trusted` falls back to the request's own values, keep only what the proxy advertised
    let scheme = match _allow_scheme_override {
        0 => None,
        _ => trusted
            .scheme()
            .filter(|scheme| Some(*scheme) != request.default_scheme())
            .map(|scheme| scheme.to_string()),
    };
    let host = match _allow_host_override {
        0 => None,
        _ => trusted
            .host_with_port()
            .filter(|host| Some(*host) != request.default_host())
            .map(|host| host.to_string()),
    };

    request.set_remote_ip(remote_ip);

    if let Some(scheme) = scheme {
        request.set_scheme(scheme);
    }

    if let Some(host) = host {
        request.set_host(host);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_request_from_str(_url: *const c_char) -> *const Request {
    let url = c_char_to_str(_url).unwrap_or("/");

    match url.parse::<Request>() {
        Err(err) => {
            tracing::error!("cannot create request for url {url}: {err}");

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

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;

    /// Replays what the nginx / apache modules do.
    fn request_from_module(headers: &[(&str, &str)], peer: &str, allow_scheme_override: u8, allow_host_override: u8) -> Box<Request> {
        request_from_module_with_host("example.com", headers, peer, allow_scheme_override, allow_host_override)
    }

    fn request_from_module_with_host(
        host: &str,
        headers: &[(&str, &str)],
        peer: &str,
        allow_scheme_override: u8,
        allow_host_override: u8,
    ) -> Box<Request> {
        let uri = CString::new("/").unwrap();
        let host = CString::new(host).unwrap();
        // what a clear text virtual host computes on its own
        let scheme = CString::new("http").unwrap();
        let method = CString::new("GET").unwrap();

        let headers: Vec<(CString, CString)> = headers
            .iter()
            .map(|(name, value)| (CString::new(*name).unwrap(), CString::new(*value).unwrap()))
            .collect();

        let mut first: *mut HeaderMap = std::ptr::null_mut();
        let mut header_map: Vec<Box<HeaderMap>> = Vec::new();

        for (name, value) in &headers {
            let mut current = Box::new(HeaderMap {
                name: name.as_ptr(),
                value: value.as_ptr(),
                next: first,
            });

            first = current.as_mut() as *mut HeaderMap;
            header_map.push(current);
        }

        let proxies = CString::new("192.168.0.0/16").unwrap();
        let trusted_proxies = redirectionio_trusted_proxies_create(proxies.as_ptr());
        let request = redirectionio_request_create(uri.as_ptr(), host.as_ptr(), scheme.as_ptr(), method.as_ptr(), first) as *mut Request;
        let peer = CString::new(peer).unwrap();

        unsafe { redirectionio_request_set_forwarded(request, peer.as_ptr(), trusted_proxies, allow_scheme_override, allow_host_override) };

        unsafe { Box::from_raw(request) }
    }

    #[test]
    fn a_default_port_in_the_host_is_dropped() {
        let request = request_from_module_with_host("example.com:443", &[("Host", "example.com:443")], "1.2.3.4", 1, 1);
        assert_eq!(request.host(), Some("example.com"));

        let request = request_from_module_with_host("example.com:8443", &[("Host", "example.com:8443")], "1.2.3.4", 1, 1);
        assert_eq!(request.host(), Some("example.com:8443"));
    }

    #[test]
    fn a_default_port_in_the_forwarded_host_is_dropped() {
        let request = request_from_module(
            &[
                ("Host", "example.com"),
                ("Forwarded", "for=1.2.3.4;proto=https;host=forwarded.example.com:443"),
            ],
            "192.168.1.1",
            1,
            1,
        );
        assert_eq!(request.host(), Some("forwarded.example.com"));

        let request = request_from_module(
            &[
                ("Host", "example.com"),
                ("Forwarded", "for=1.2.3.4;proto=https;host=forwarded.example.com:8443"),
            ],
            "192.168.1.1",
            1,
            1,
        );
        assert_eq!(request.host(), Some("forwarded.example.com:8443"));
    }

    #[test]
    fn without_any_forwarding_header_nothing_is_touched() {
        let request = request_from_module(&[("Host", "example.com")], "192.168.1.1", 1, 1);

        assert_eq!(request.scheme(), Some("http"));
        assert_eq!(request.host(), Some("example.com"));
        assert_eq!(request.remote_addr, Some("192.168.1.1".parse().unwrap()));
    }

    #[test]
    fn trusted_proxy_forwarded_header_gives_ip_scheme_and_host() {
        let request = request_from_module(
            &[
                ("Host", "example.com"),
                ("Forwarded", "for=1.2.3.4;proto=https;host=forwarded.example.com"),
            ],
            "192.168.1.1",
            1,
            1,
        );

        assert_eq!(request.scheme(), Some("https"));
        assert_eq!(request.host(), Some("forwarded.example.com"));
        assert_eq!(request.remote_addr, Some("1.2.3.4".parse().unwrap()));
    }

    #[test]
    fn an_element_the_client_prepended_is_ignored() {
        // haproxy appends its own element, only that last one has a trusted address
        let request = request_from_module(
            &[
                ("Host", "example.com"),
                ("Forwarded", "proto=https;host=evil.example.com,proto=http;for=1.2.3.4"),
            ],
            "192.168.1.1",
            1,
            1,
        );

        assert_eq!(request.scheme(), Some("http"));
        assert_eq!(request.host(), Some("example.com"));
        assert_eq!(request.remote_addr, Some("1.2.3.4".parse().unwrap()));
    }

    #[test]
    fn x_forwarded_proto_and_host_are_not_trusted() {
        let request = request_from_module(
            &[
                ("Host", "example.com"),
                ("X-Forwarded-For", "1.2.3.4"),
                ("X-Forwarded-Proto", "https"),
                ("X-Forwarded-Host", "forwarded.example.com"),
            ],
            "192.168.1.1",
            1,
            1,
        );

        // the ip still comes from X-Forwarded-For, the scheme and the host do not follow
        assert_eq!(request.remote_addr, Some("1.2.3.4".parse().unwrap()));
        assert_eq!(request.scheme(), Some("http"));
        assert_eq!(request.host(), Some("example.com"));
    }

    #[test]
    fn an_untrusted_peer_is_never_believed() {
        let request = request_from_module(
            &[
                ("Host", "example.com"),
                ("Forwarded", "for=1.2.3.4;proto=https;host=forwarded.example.com"),
            ],
            "8.8.8.8",
            1,
            1,
        );

        assert_eq!(request.scheme(), Some("http"));
        assert_eq!(request.host(), Some("example.com"));
        assert_eq!(request.remote_addr, Some("8.8.8.8".parse().unwrap()));
    }

    #[test]
    fn an_explicitly_configured_scheme_and_host_win() {
        let request = request_from_module(
            &[
                ("Host", "example.com"),
                ("Forwarded", "for=1.2.3.4;proto=https;host=forwarded.example.com"),
            ],
            "192.168.1.1",
            0,
            0,
        );

        assert_eq!(request.scheme(), Some("http"));
        assert_eq!(request.host(), Some("example.com"));
        // the ip is always taken from a trusted proxy
        assert_eq!(request.remote_addr, Some("1.2.3.4".parse().unwrap()));
    }

    #[test]
    fn set_remote_addr_keeps_its_ip_only_behaviour() {
        let uri = CString::new("/").unwrap();
        let host = CString::new("example.com").unwrap();
        let scheme = CString::new("http").unwrap();
        let method = CString::new("GET").unwrap();
        let name = CString::new("Forwarded").unwrap();
        let value = CString::new("for=1.2.3.4;proto=https;host=forwarded.example.com").unwrap();
        let mut header = Box::new(HeaderMap {
            name: name.as_ptr(),
            value: value.as_ptr(),
            next: std::ptr::null_mut(),
        });

        let proxies = CString::new("192.168.0.0/16").unwrap();
        let trusted_proxies = redirectionio_trusted_proxies_create(proxies.as_ptr());
        let request = redirectionio_request_create(
            uri.as_ptr(),
            host.as_ptr(),
            scheme.as_ptr(),
            method.as_ptr(),
            header.as_mut() as *mut HeaderMap,
        ) as *mut Request;
        let peer = CString::new("192.168.1.1").unwrap();

        unsafe { redirectionio_request_set_remote_addr(request, peer.as_ptr(), trusted_proxies) };

        let request = unsafe { Box::from_raw(request) };

        assert_eq!(request.remote_addr, Some("1.2.3.4".parse().unwrap()));
        assert_eq!(request.scheme(), Some("http"));
        assert_eq!(request.host(), Some("example.com"));
    }
}
