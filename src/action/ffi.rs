use crate::action::Action;
use crate::api::MessageHeader;
use crate::filter::{FilterHeaderAction, FilterBodyAction};
use crate::ffi_helpers::{c_char_to_str, string_to_c_char};
use std::os::raw::c_char;
use std::ptr::null;

#[no_mangle]
pub unsafe extern fn redirectionio_action_get_status_code(_action: *const Action, response_status_code: u16) -> u16 {
    if _action.is_null() {
        return 0;
    }

    let action = &*_action;

    match action.status_code_update.as_ref() {
        None => 0,
        Some(status_code_update) => {
            status_code_update.get_status_code(response_status_code)
        }
    }
}

#[no_mangle]
pub unsafe extern fn redirectionio_action_replace_headers(_action: *const Action, _headers: *mut Vec<MessageHeader>, response_status_code: u16) {
    if _action.is_null() || _headers.is_null() {
        return;
    }

    let action = &*_action;
    let box_headers = Box::from_raw(_headers);
    let mut headers = Vec::new();

    for header in box_headers.iter() {
        headers.push(header.clone());
    }

    headers = action.filter_headers(headers, response_status_code);
}

#[no_mangle]
pub unsafe extern fn redirectionio_action_create_body_filter(_action: *const Action, response_status_code: u16) -> Option<*mut FilterBodyAction> {
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
pub unsafe extern fn redirectionio_action_body_filter(_filter: *mut FilterBodyAction, _body: *const c_char) -> *const c_char {
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
pub unsafe extern fn redirectionio_action_end_body_filter(_filter: *mut FilterBodyAction) -> *const c_char {
    if _filter.is_null() {
        return null();
    }

    let mut filter = Box::from_raw(_filter);
    let end_body = filter.end();

    string_to_c_char(end_body)
}


