use crate::router::{Route, Router};
use crate::api::{Rule, RulesMessage};
use crate::http::Request;
use crate::action::Action;
use crate::ffi_helpers::c_char_to_str;
use std::os::raw::{c_char, c_ulong};
use std::ptr::null;

#[no_mangle]
pub unsafe extern fn redirectionio_route_create(s: *const c_char) -> Option<*mut Route<Rule>> {
    let rule_string = c_char_to_str(s)?;
    let route = Rule::from_str(rule_string)?.to_route();

    Some(Box::into_raw(Box::new(route)))
}

#[no_mangle]
pub unsafe extern fn redirectionio_router_create(_message: *mut RulesMessage, cache: c_ulong) -> *mut Router<Rule> {
    let mut router = Router::<Rule>::new();

    if !_message.is_null() {
        let message= Box::from_raw(_message);

        for rule in message.rules {
            router.insert(rule.to_route());
        }
    }

    router.cache(cache);

    Box::into_raw(Box::new(router))
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
pub unsafe extern fn redirectionio_router_match_action(_router: *const Router<Rule>, _request: *const Request) -> *const Action {
    if _router.is_null() || _request.is_null() {
        return null() as *const Action;
    }

    let router = &*_router;
    let request = &*_request;

    let http_request = match request.to_http_request() {
        Err(error) => {
            error!("{}", error);

            return null() as *const Action;
        },
        Ok(request) => request,
    };
    let routes = router.match_request(&http_request);
    let action = Action::from_routes_rule(routes, &http_request);

    Box::into_raw(Box::new(action))
}

#[no_mangle]
pub unsafe extern fn redirectionio_router_len(_router: *const Router<Rule>) -> c_ulong {
    if _router.is_null() {
        return 0;
    }

    let router = &*_router;

    router.len() as c_ulong
}