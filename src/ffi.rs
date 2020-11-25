use crate::action::Action;
use crate::api::{Impact, ImpactResultItem, RouterTrace, Rule, RulesMessage};
use crate::ffi_helpers::c_char_to_str;
use crate::http::Request;
use crate::router::{Route, Router};
use std::os::raw::{c_char, c_ulong};
use std::ptr::null;

#[no_mangle]
pub unsafe extern "C" fn redirectionio_route_create(s: *const c_char) -> *const Route<Rule> {
    let rule_string = c_char_to_str(s);

    match rule_string {
        None => null() as *const Route<Rule>,
        Some(str) => {
            match Rule::from_json(str) {
                None => null() as *const Route<Rule>,
                Some(rule) => Box::into_raw(Box::new(rule.into_route()))
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_router_create(_message: *mut RulesMessage, cache: u64) -> *mut Router<Rule> {
    let mut router = Router::<Rule>::default();

    if !_message.is_null() {
        let message = Box::from_raw(_message);

        for rule in message.rules {
            router.insert(rule.into_route());
        }
    }

    router.cache(cache);

    Box::into_raw(Box::new(router))
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_router_route_add(_route: *mut Route<Rule>, _router: *mut Router<Rule>) {
    let route = Box::from_raw(_route);
    let router = &mut *_router;

    router.insert(*route);
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_router_route_remove(_route_id: *const c_char, _router: *mut Router<Rule>) {
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
pub unsafe extern "C" fn redirectionio_router_drop(_router: *mut Router<Rule>) {
    if _router.is_null() {
        return;
    }

    Box::from_raw(_router);
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_router_match_action(_router: *const Router<Rule>, _request: *const Request) -> *const Action {
    if _router.is_null() || _request.is_null() {
        return null() as *const Action;
    }

    let router = &*_router;
    let request = &*_request;

    let http_request = match request.to_http_request() {
        Err(error) => {
            error!("{}", error);

            return null() as *const Action;
        }
        Ok(request) => request,
    };
    let routes = router.match_request(&http_request);
    let action = Action::from_routes_rule(routes, request);

    Box::into_raw(Box::new(action))
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_router_trace(_router: *const Router<Rule>, _request: *const Request) -> *const RouterTrace {
    if _router.is_null() || _request.is_null() {
        return null() as *const RouterTrace;
    }

    let router = &*_router;
    let request = &*_request;

    let http_request = match request.to_http_request() {
        Err(error) => {
            error!("{}", error);

            return null() as *const RouterTrace;
        }
        Ok(request) => request,
    };

    let trace = RouterTrace::create_from_router(router, request, &http_request);

    Box::into_raw(Box::new(trace))
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_router_impact(_router: *const Router<Rule>, _impact: *const Impact) -> *const Vec<ImpactResultItem> {
    if _router.is_null() {
        return null() as *const Vec<ImpactResultItem>;
    }

    let router = &*_router;
    let impact = &*_impact;

    let result = Impact::create_result(router, impact);

    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_router_len(_router: *const Router<Rule>) -> c_ulong {
    if _router.is_null() {
        return 0;
    }

    let router = &*_router;

    router.len() as c_ulong
}
