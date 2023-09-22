use crate::action::Action;
use crate::api::{Rule, RulesMessage};
use crate::ffi_helpers::c_char_to_str;
use crate::http::Request;
use crate::router::{Route, Router, RouterConfig};
use std::os::raw::{c_char, c_ulong};
use std::ptr::null;

#[no_mangle]
pub unsafe extern "C" fn redirectionio_router_create(
    config_serialized: *mut c_char,
    _message: *mut RulesMessage,
    cache: u64,
) -> *mut Router<Rule> {
    let config = match c_char_to_str(config_serialized) {
        None => RouterConfig::default(),
        Some(str) => {
            if str.is_empty() {
                RouterConfig::default()
            } else {
                match serde_json::from_str(str) {
                    Ok(value) => value,
                    Err(error) => {
                        error!("Unable to deserialize router config: {}", error,);

                        RouterConfig::default()
                    }
                }
            }
        }
    };

    let mut router = Router::<Rule>::from_config(config);

    if !_message.is_null() {
        let message = Box::from_raw(_message);

        for rule in message.rules {
            router.insert(rule.into_route(&router.config));
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

    drop(Box::from_raw(_router));
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_router_match_action(_router: *const Router<Rule>, _request: *const Request) -> *const Action {
    if _router.is_null() || _request.is_null() {
        return null();
    }

    let router = &*_router;
    let request = &*_request;

    let request_rebuild = router.rebuild_request(request);
    let routes = router.match_request(&request_rebuild);
    let action = Action::from_routes_rule(routes, &request_rebuild, None);

    Box::into_raw(Box::new(action))
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_router_len(_router: *const Router<Rule>) -> c_ulong {
    if _router.is_null() {
        return 0;
    }

    let router = &*_router;

    router.len() as c_ulong
}
