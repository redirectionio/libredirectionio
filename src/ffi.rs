use crate::router::{Route, Router};
use crate::api::{Rule, RulesMessage};
use crate::ffi_helpers::{c_char_to_str, c_char_to_string};
use std::ptr::null;
use std::ffi::CStr;
use std::os::raw::{c_char, c_ulong};

#[no_mangle]
pub unsafe extern fn redirectionio_create_route(s: *const c_char) -> Option<*mut Route<Rule>> {
    let rule_string = c_char_to_str(s)?;
    let route = Rule::from_str(rule_string)?.to_route();

    Some(Box::into_raw(Box::new(route)))
}

#[no_mangle]
pub unsafe extern fn redirectionio_create_router(_message: *mut RulesMessage, cache: c_ulong) -> Option<*mut Router<Rule>> {
    if _message.is_null() {
        return None;
    }

    let message= Box::from_raw(_message);
    let mut router = Router::<Rule>::new();

    for rule in message.rules {
        router.insert(rule.to_route());
    }

    router.cache(cache);

    Some(Box::into_raw(Box::new(router)))
}

#[no_mangle]
pub unsafe extern fn redirectionio_router_insert_route(_route: *mut Route<Rule>, _router: *mut Router<Rule>) {
    let route = Box::from_raw(_route);
    let router = &mut *_router;

    router.insert(*route);
}

#[no_mangle]
pub unsafe extern fn redirectionio_router_remove_route(_route_id: *const c_char, _router: *mut Router<Rule>) {
    if _router.is_null() {
        return;
    }

    let route_id = c_char_to_str(_route_id);

    if route_id.is_none() {
        return;
    }

    let router = &mut *_router;

    router.remove(route_id.unwrap());
}

#[no_mangle]
pub unsafe extern fn redirectionio_drop_router(_router: *mut Router<Rule>) {
    if _router.is_null() {
        return;
    }

    Box::from_raw(_router);
}
