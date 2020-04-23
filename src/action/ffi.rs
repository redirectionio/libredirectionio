use crate::action::Action;
use crate::filter::FilterBodyAction;
use crate::ffi_helpers::{c_char_to_str, string_to_c_char};
use crate::http::Header;
use std::os::raw::c_char;
use std::ptr::null;
use serde_json::{from_str as json_decode, to_string as json_encode};

#[no_mangle]
/// Deserialize a string to an action
///
/// Returns null if an error happens, otherwise it returns a pointer to an action
pub unsafe extern fn redirectionio_action_json_deserialize(str: *mut c_char) -> Option<*mut Action> {
    let action_str = c_char_to_str(str)?;

    let action = match json_decode(action_str) {
        Err(error) => {
            error!(
                "Unable to deserialize \"{}\" to action: {}",
                action_str,
                error,
            );

            return None;
        },
        Ok(action) => action,
    };

    Some(Box::into_raw(Box::new(action)))
}

#[no_mangle]
/// Serialize an action to a string
///
/// Returns null if an error happens
pub unsafe extern fn redirectionio_action_json_serialize(_action: *mut Action) -> *const c_char {
    if _action.is_null() {
        return null();
    }

    let action = &*_action;
    let action_serialized = match json_encode(action) {
        Err(error) => {
            error!(
                "Unable to serialize to action: {}",
                error,
            );

            return null();
        },
        Ok(action_serialized) => action_serialized,
    };

    string_to_c_char(action_serialized)
}

#[no_mangle]
pub unsafe extern fn redirectionio_action_drop(_action: *mut Action) {
    if _action.is_null() {
        return;
    }

    Box::from_raw(_action);
}

#[no_mangle]
pub unsafe extern fn redirectionio_action_get_status_code(_action: *const Action, response_status_code: u16) -> u16 {
    if _action.is_null() {
        return 0;
    }

    let action = &*_action;

    action.get_status_code(response_status_code)
}

#[no_mangle]
pub unsafe extern fn redirectionio_action_header_filter_filter(_action: *const Action, _headers: *mut Vec<Header>, response_status_code: u16) -> Option<*mut Vec<Header>> {
    if _action.is_null() || _headers.is_null() {
        return None;
    }

    let action = &*_action;
    let box_headers = Box::from_raw(_headers);
    let mut headers = Vec::new();

    for header in box_headers.iter() {
        headers.push(header.clone());
    }

    headers = action.filter_headers(headers, response_status_code);

    Some(Box::into_raw(Box::new(headers)))
}

#[no_mangle]
pub unsafe extern fn redirectionio_action_body_filter_create(_action: *const Action, response_status_code: u16) -> Option<*mut FilterBodyAction> {
    if _action.is_null() {
        return None;
    }

    let action = &*_action;

    match action.create_filter_body(response_status_code) {
        None => None,
        Some(filter_body) => Some(Box::into_raw(Box::new(filter_body)))
    }
}

#[no_mangle]
pub unsafe extern fn redirectionio_action_body_filter_filter(_filter: *mut FilterBodyAction, _body: *const c_char) -> *const c_char {
    if _filter.is_null() {
        return _body;
    }

    let filter = &mut *_filter;
    let body = match c_char_to_str(_body) {
        None => return _body,
        Some(str) => str,
    };

    let new_body = filter.filter(body.to_string());

    string_to_c_char(new_body)
}

#[no_mangle]
pub unsafe extern fn redirectionio_action_body_filter_close(_filter: *mut FilterBodyAction) -> *const c_char {
    if _filter.is_null() {
        return null();
    }

    let mut filter = Box::from_raw(_filter);
    let end_body = filter.end();

    string_to_c_char(end_body)
}


