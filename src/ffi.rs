use crate::router::{Route, Router};
use crate::api::{Rule, RulesMessage};
use crate::http::Request;
use crate::action::Action;
use crate::ffi_helpers::c_char_to_str;
use std::os::raw::{c_char, c_ulong};

#[no_mangle]
pub unsafe extern fn redirectionio_route_create(s: *const c_char) -> Option<*mut Route<Rule>> {
    let rule_string = c_char_to_str(s)?;
    let route = Rule::from_str(rule_string)?.to_route();

    Some(Box::into_raw(Box::new(route)))
}

#[no_mangle]
pub unsafe extern fn redirectionio_router_create(_message: *mut RulesMessage, cache: c_ulong) -> Option<*mut Router<Rule>> {
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
pub unsafe extern fn redirectionio_router_route_add(_route: *mut Route<Rule>, _router: *mut Router<Rule>) {
    let route = Box::from_raw(_route);
    let router = &mut *_router;

    router.insert(*route);
}

#[no_mangle]
pub unsafe extern fn redirectionio_router_route_remove(_route_id: *const c_char, _router: *mut Router<Rule>) {
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
pub unsafe extern fn redirectionio_router_drop(_router: *mut Router<Rule>) {
    if _router.is_null() {
        return;
    }

    Box::from_raw(_router);
}

#[no_mangle]
pub unsafe extern fn redirectionio_router_match_action(_router: *const Router<Rule>, _request: *const Request) -> *mut Action {
    let mut action = Action::new();

    if _router.is_null() || _request.is_null() {
        return Box::into_raw(Box::new(action));
    }

    let router = &*_router;
    let request = &*_request;

    let http_request = request.to_http_request();
    let routes = router.match_request(&http_request);

    action = Action::from_routes_rule(routes, &http_request);

    Box::into_raw(Box::new(action))
}
