use std::{os::raw::c_char, ptr::null};

use serde_json::{from_str as json_decode, to_string as json_encode};

use crate::{
    action::Action,
    ffi_helpers::{c_char_to_str, string_to_c_char},
    filter::{Buffer, FilterBodyAction},
    http::ffi::{
        HeaderMap, header_map_to_http_headers, header_map_to_http_headers_keeping_raw, http_headers_to_header_map,
        http_headers_to_header_map_with_raw,
    },
};

/// Deserialize a string to an action
///
/// Returns null if an error happens, otherwise it returns a pointer to an action
#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_action_json_deserialize(str: *mut c_char) -> *const Action {
    let action_str = match c_char_to_str(str) {
        None => return null(),
        Some(str) => str,
    };

    let action = match json_decode(action_str) {
        Err(error) => {
            tracing::error!("unable to deserialize \"{action_str}\" to action: {error}");

            return null();
        }
        Ok(action) => action,
    };

    Box::into_raw(Box::new(action))
}

/// Serialize an action to a string
///
/// Returns null if an error happens
#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_action_json_serialize(_action: *mut Action) -> *const c_char {
    if _action.is_null() {
        return null();
    }

    // SAFETY: _action is a valid pointer to an Action
    let action = unsafe { &*_action };
    let action_serialized = match json_encode(action) {
        Err(error) => {
            tracing::error!("unable to serialize to action: {error}");

            return null();
        }
        Ok(action_serialized) => action_serialized,
    };

    string_to_c_char(action_serialized)
}

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_action_drop(_action: *mut Action) {
    if _action.is_null() {
        return;
    }

    // SAFETY: _action is a valid pointer to an Action
    drop(unsafe { Box::from_raw(_action) });
}

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_action_get_status_code(_action: *mut Action, response_status_code: u16) -> u16 {
    if _action.is_null() {
        return 0;
    }

    // SAFETY: _action is a valid pointer to an Action
    let action = unsafe { &mut *_action };

    action.get_status_code(response_status_code, None)
}

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_action_header_filter_filter(
    _action: *mut Action,
    header_map: *const HeaderMap,
    response_status_code: u16,
    add_rule_ids_header: bool,
) -> *const HeaderMap {
    if _action.is_null() {
        return header_map;
    }

    // SAFETY: _action is a valid pointer to an Action
    let action = unsafe { &mut *_action };
    let mut headers = header_map_to_http_headers(header_map);

    headers = action.filter_headers(headers, response_status_code, add_rule_ids_header, None);

    http_headers_to_header_map(headers)
}

/// Filter the headers of the request forwarded to the backend.
///
/// Returns null when the action has no request header filter, in which case the request
/// must be left untouched. Otherwise returns the complete new list of request headers,
/// in the same order as the input list, which replaces the original one. The returned
/// list must be freed with `redirectionio_header_map_drop`.
#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_action_request_header_filter_filter(
    _action: *mut Action,
    header_map: *const HeaderMap,
) -> *const HeaderMap {
    if _action.is_null() {
        return null();
    }

    // SAFETY: _action is a valid pointer to an Action
    let action = unsafe { &mut *_action };

    if !action.has_request_header_filters() {
        return null();
    }

    // The returned list replaces the request headers: the ones the filters cannot read are
    // forwarded untouched rather than dropped.
    let (headers, raw_headers) = header_map_to_http_headers_keeping_raw(header_map);
    let headers = action.filter_request_headers(headers, None);

    http_headers_to_header_map_with_raw(headers, raw_headers)
}

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_action_body_filter_create(
    _action: *mut Action,
    response_status_code: u16,
    response_header_map: *const HeaderMap,
) -> *const FilterBodyAction {
    if _action.is_null() {
        return null();
    }

    // SAFETY: _action is a valid pointer to an Action
    let action = unsafe { &mut *_action };
    let headers = header_map_to_http_headers(response_header_map);

    match action.create_filter_body(response_status_code, headers.as_ref(), None) {
        None => null(),
        Some(filter_body) => Box::into_raw(Box::new(filter_body)),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_action_body_filter_filter(_filter: *mut FilterBodyAction, buffer: Buffer) -> Buffer {
    if _filter.is_null() {
        // No filter: pass the buffer straight through. Move it out rather than
        // duplicating, otherwise the original allocation would leak (Buffer has
        // no Drop impl, so a borrowed-and-copied input is never reclaimed).
        return buffer;
    }

    // SAFETY: _filter is a valid pointer to a FilterBodyAction
    let filter = unsafe { &mut *_filter };
    let bytes = buffer.into_vec();

    let new_body = filter.filter(bytes, None);

    Buffer::from_vec(new_body)
}

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_action_body_filter_close(_filter: *mut FilterBodyAction) -> Buffer {
    if _filter.is_null() {
        return Buffer::default();
    }

    // SAFETY: _filter is a valid pointer to a FilterBodyAction
    let filter = unsafe { Box::from_raw(_filter) };
    let end_body = filter.end(None);

    Buffer::from_vec(end_body)
}

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_action_body_filter_drop(_filter: *mut FilterBodyAction) {
    if _filter.is_null() {
        return;
    }

    // SAFETY: _filter is a valid pointer to a FilterBodyAction
    drop(unsafe { Box::from_raw(_filter) });
}

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_action_should_log_request(_action: *mut Action, allow_log_config: bool, response_status_code: u16) -> bool {
    if _action.is_null() {
        return allow_log_config;
    }

    // SAFETY: _action is a valid pointer to an Action
    let action = unsafe { &mut *_action };

    action.should_log_request(allow_log_config, response_status_code, None)
}

/// Serialize the ids of the rules applied by this action as a JSON array of strings.
///
/// Used to count rule executions when logging is disabled for the request (so no log
/// is sent). Returns null when no rule was applied. The returned string must be freed
/// with `redirectionio_string_drop`.
#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_action_get_applied_rule_ids(_action: *mut Action) -> *const c_char {
    if _action.is_null() {
        return null();
    }

    // SAFETY: _action is a valid pointer to an Action
    let action = unsafe { &*_action };
    let rule_ids = action.get_applied_rule_ids_vec();

    if rule_ids.is_empty() {
        return null();
    }

    match json_encode(&rule_ids) {
        Err(error) => {
            tracing::error!("unable to serialize applied rule ids: {error}");

            null()
        }
        Ok(serialized) => string_to_c_char(serialized),
    }
}

/// Whether the agent that produced this action understands the `RULE_COUNT` command
/// (protocol >= 1.1), as advertised in the MATCH response. A proxy module must check this
/// before sending `RULE_COUNT`, so it never sends it to an older agent that would reject
/// the unknown command and close the connection. Returns false on a null action or when
/// the agent advertised no version (older agent).
#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_action_agent_supports_rule_count(_action: *mut Action) -> bool {
    if _action.is_null() {
        return false;
    }

    // SAFETY: _action is a valid pointer to an Action
    let action = unsafe { &*_action };

    action.agent_supports_rule_count()
}

#[cfg(test)]
mod request_header_filter_tests {
    use std::ffi::CString;

    use super::redirectionio_action_request_header_filter_filter;
    use crate::{
        action::Action,
        http::{
            Header,
            ffi::{header_map_to_http_headers_keeping_raw, http_headers_to_header_map_with_raw, redirectionio_header_map_drop},
        },
    };

    const BASE: &str =
        r#""status_code_update":null,"header_filters":[],"body_filters":[],"rule_ids":[],"log_override":null,"peer_override":null"#;

    fn header(name: &str, value: &str) -> Header {
        Header {
            name: name.to_string(),
            value: value.to_string(),
        }
    }

    #[test]
    fn returns_null_without_request_header_filters() {
        let action = Box::into_raw(Box::new(serde_json::from_str::<Action>(&format!("{{{BASE}}}")).unwrap()));
        let input = http_headers_to_header_map_with_raw(vec![header("X-Foo", "foo")], Vec::new());

        assert!(redirectionio_action_request_header_filter_filter(action, input).is_null());

        unsafe {
            redirectionio_header_map_drop(input);
            drop(Box::from_raw(action));
        }
    }

    #[test]
    fn keeps_order_and_forwards_non_utf8_headers() {
        let json = format!(
            r#"{{{BASE},"request_header_filters":[{{"filter":{{"action":"override","header":"X-Second","value":"new","id":null,"target_hash":null}},"rule_id":"rule"}}]}}"#
        );
        let action = Box::into_raw(Box::new(serde_json::from_str::<Action>(&json).unwrap()));
        let raw = (CString::new("X-Raw").unwrap(), CString::new(vec![0xff, 0xfe]).unwrap());
        let input = http_headers_to_header_map_with_raw(vec![header("X-First", "1"), header("X-Second", "2")], vec![raw.clone()]);

        let output = redirectionio_action_request_header_filter_filter(action, input);
        let (headers, raw_headers) = header_map_to_http_headers_keeping_raw(output);

        assert_eq!(
            headers.into_iter().map(|h| (h.name, h.value)).collect::<Vec<_>>(),
            vec![
                ("X-First".to_string(), "1".to_string()),
                ("X-Second".to_string(), "new".to_string())
            ]
        );
        assert_eq!(raw_headers, vec![raw]);

        unsafe {
            redirectionio_header_map_drop(input);
            redirectionio_header_map_drop(output);
            drop(Box::from_raw(action));
        }
    }
}
