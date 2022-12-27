use crate::action::Action;
use crate::ffi_helpers::{c_char_to_str, string_to_c_char};
use crate::filter::{Buffer, FilterBodyAction};
use crate::http::ffi::{header_map_to_http_headers, http_headers_to_header_map, HeaderMap};
use serde_json::{from_str as json_decode, to_string as json_encode};
use std::os::raw::c_char;
use std::ptr::null;

#[no_mangle]
/// Deserialize a string to an action
///
/// Returns null if an error happens, otherwise it returns a pointer to an action
pub unsafe extern "C" fn redirectionio_action_json_deserialize(str: *mut c_char) -> *const Action {
    let action_str = match c_char_to_str(str) {
        None => return null() as *const Action,
        Some(str) => str,
    };

    let action = match json_decode(action_str) {
        Err(error) => {
            error!("Unable to deserialize \"{}\" to action: {}", action_str, error,);

            return null() as *const Action;
        }
        Ok(action) => action,
    };

    Box::into_raw(Box::new(action))
}

#[no_mangle]
/// Serialize an action to a string
///
/// Returns null if an error happens
pub unsafe extern "C" fn redirectionio_action_json_serialize(_action: *mut Action) -> *const c_char {
    if _action.is_null() {
        return null();
    }

    let action = &*_action;
    let action_serialized = match json_encode(action) {
        Err(error) => {
            error!("Unable to serialize to action: {}", error,);

            return null();
        }
        Ok(action_serialized) => action_serialized,
    };

    string_to_c_char(action_serialized)
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_action_drop(_action: *mut Action) {
    if _action.is_null() {
        return;
    }

    drop(Box::from_raw(_action));
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_action_get_status_code(_action: *mut Action, response_status_code: u16) -> u16 {
    if _action.is_null() {
        return 0;
    }

    let action = &mut *_action;

    action.get_status_code(response_status_code, None)
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_action_header_filter_filter(
    _action: *mut Action,
    header_map: *const HeaderMap,
    response_status_code: u16,
    add_rule_ids_header: bool,
) -> *const HeaderMap {
    if _action.is_null() {
        return header_map;
    }

    let action = &mut *_action;
    let mut headers = header_map_to_http_headers(header_map);

    headers = action.filter_headers(headers, response_status_code, add_rule_ids_header, None);

    http_headers_to_header_map(headers)
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_action_body_filter_create(
    _action: *mut Action,
    response_status_code: u16,
    response_header_map: *const HeaderMap,
) -> *const FilterBodyAction {
    if _action.is_null() {
        return null() as *const FilterBodyAction;
    }

    let action = &mut *_action;
    let headers = header_map_to_http_headers(response_header_map);

    match action.create_filter_body(response_status_code, &headers) {
        None => null(),
        Some(filter_body) => Box::into_raw(Box::new(filter_body)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_action_body_filter_filter(_filter: *mut FilterBodyAction, buffer: Buffer) -> Buffer {
    if _filter.is_null() {
        return buffer.duplicate();
    }

    let filter = &mut *_filter;
    let bytes = buffer.into_vec();

    let new_body = filter.filter(bytes, None);

    Buffer::from_vec(new_body)
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_action_body_filter_close(_filter: *mut FilterBodyAction) -> Buffer {
    if _filter.is_null() {
        return Buffer::default();
    }

    let mut filter = Box::from_raw(_filter);
    let end_body = filter.end(None);
    drop(filter);

    Buffer::from_vec(end_body)
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_action_body_filter_drop(_filter: *mut FilterBodyAction) {
    if _filter.is_null() {
        return;
    }

    drop(Box::from_raw(_filter));
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_action_should_log_request(
    _action: *mut Action,
    allow_log_config: bool,
    response_status_code: u16,
) -> bool {
    if _action.is_null() {
        return allow_log_config;
    }

    let action = &mut *_action;

    action.should_log_request(allow_log_config, response_status_code, None)
}
