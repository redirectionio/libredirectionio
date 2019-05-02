extern crate cfg_if;
extern crate wasm_bindgen;
#[macro_use]
extern crate lazy_static;
extern crate libc;

mod api;
mod filter;
mod html;
mod router;
mod utils;

use cfg_if::cfg_if;
use std::collections::HashMap;
use std::intrinsics::transmute;
use std::ptr::null;
use std::sync::Mutex;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

lazy_static! {
    static ref PROJECT_ROUTERS: Mutex<HashMap<String, router::MainRouter>> =
        Mutex::new(HashMap::new());
    static ref FILTERS: Mutex<HashMap<String, filter::filter_body::FilterBodyAction>> =
        Mutex::new(HashMap::new());
}

#[wasm_bindgen]
pub fn update_rules_for_router(project_id: String, rules_data: String, cache: bool) -> String {
    utils::set_panic_hook();
    let main_router = router::MainRouter::new_from_data(rules_data, cache);

    PROJECT_ROUTERS
        .lock()
        .unwrap()
        .insert(project_id.clone(), main_router);

    return project_id;
}

#[no_mangle]
pub extern "C" fn redirectionio_update_rules_for_router(
    project_id_cstr: *const libc::c_char,
    rules_data_cstr: *const libc::c_char,
    cache: libc::c_uint,
) -> *const libc::c_char {
    unsafe {
        let project_id = std::ffi::CStr::from_ptr(project_id_cstr)
            .to_str()
            .expect("Cannot create string")
            .to_string();
        let rules_data = std::ffi::CStr::from_ptr(rules_data_cstr)
            .to_str()
            .expect("Cannot create string")
            .to_string();

        let project_id_created = update_rules_for_router(project_id, rules_data, cache != 0);

        return str_to_cstr(project_id_created);
    }
}

#[wasm_bindgen]
pub fn get_rule_for_url(project_id: String, url: String) -> Option<String> {
    let lock = PROJECT_ROUTERS.lock();
    let router: Option<&router::MainRouter> = lock.as_ref().unwrap().get(project_id.as_str());

    if router.is_none() {
        return None;
    }

    let rule = router.unwrap().match_rule(url);

    if rule.is_none() {
        return None;
    }

    Some(rule_to_string(rule.unwrap()))
}

#[no_mangle]
pub extern "C" fn redirectionio_get_rule_for_url(
    project_id_cstr: *const libc::c_char,
    url_cstr: *const libc::c_char,
) -> *const libc::c_char {
    unsafe {
        let project_id = std::ffi::CStr::from_ptr(project_id_cstr)
            .to_str()
            .expect("Cannot create string")
            .to_string();
        let url = std::ffi::CStr::from_ptr(url_cstr)
            .to_str()
            .expect("Cannot create string")
            .to_string();

        let rule_data = get_rule_for_url(project_id, url);

        if rule_data.is_none() {
            return null();
        }

        return str_to_cstr(rule_data.unwrap());
    }
}

#[wasm_bindgen]
pub fn get_redirect(rule_str: String, url: String) -> Option<String> {
    if rule_str.is_empty() {
        return None;
    }

    let rule = string_to_rule(rule_str);

    if rule.is_none() {
        return None;
    }

    let rule_obj = rule.unwrap();

    if rule_obj.redirect_code == 0 {
        return None;
    }

    let target = router::MainRouter::get_redirect(&rule_obj, url);
    let redirect = api::Redirect {
        status: rule_obj.redirect_code,
        target,
    };

    return Some(serde_json::to_string(&redirect).expect("Cannot serialize redirect"));
}

#[no_mangle]
pub extern "C" fn redirectionio_get_redirect(
    rule_cstr: *const libc::c_char,
    url_cstr: *const libc::c_char,
) -> *const libc::c_char {
    unsafe {
        let rule = std::ffi::CStr::from_ptr(rule_cstr)
            .to_str()
            .expect("Cannot create string")
            .to_string();
        let url = std::ffi::CStr::from_ptr(url_cstr)
            .to_str()
            .expect("Cannot create string")
            .to_string();

        let redirect = get_redirect(rule, url);

        if redirect.is_none() {
            return null();
        }

        return str_to_cstr(redirect.unwrap());
    }
}

#[wasm_bindgen]
pub fn header_filter(rule_str: String, headers_str: String) -> String {
    let rule = string_to_rule(rule_str);

    if rule.is_none() {
        return headers_str;
    }

    let rule_obj = rule.unwrap();
    let filter = filter::filter_header::FilterHeaderAction::new(rule_obj);

    if filter.is_none() {
        return headers_str;
    }

    let headers: Option<Vec<filter::header_action::Header>> =
        serde_json::from_str(&headers_str).unwrap();

    if headers.is_none() {
        return headers_str;
    }

    let new_headers = filter.unwrap().filter(headers.unwrap());

    return serde_json::to_string(&new_headers).expect("Cannot serialize headers");
}

#[no_mangle]
pub extern "C" fn redirectionio_header_filter(
    rule_cstr: *const libc::c_char,
    headers_cstr: *const libc::c_char,
) -> *const libc::c_char {
    unsafe {
        let rule = std::ffi::CStr::from_ptr(rule_cstr)
            .to_str()
            .expect("Cannot create string")
            .to_string();
        let headers = std::ffi::CStr::from_ptr(headers_cstr)
            .to_str()
            .expect("Cannot create string")
            .to_string();

        let new_headers_str = header_filter(rule, headers);

        return str_to_cstr(new_headers_str);
    }
}

#[wasm_bindgen]
pub fn create_body_filter(rule_str: String) -> Option<String> {
    let rule = string_to_rule(rule_str);

    if rule.is_none() {
        return None;
    }

    let rule_obj = rule.unwrap();

    let filter = filter::filter_body::FilterBodyAction::new(rule_obj);

    if filter.is_none() {
        return None;
    }

    let uuid = Uuid::new_v4().to_string();

    FILTERS
        .lock()
        .unwrap()
        .insert(uuid.clone(), filter.unwrap());

    return Some(uuid);
}

#[no_mangle]
pub extern "C" fn redirectionio_create_body_filter(
    rule_cstr: *const libc::c_char,
) -> *const libc::c_char {
    unsafe {
        let rule = std::ffi::CStr::from_ptr(rule_cstr)
            .to_str()
            .expect("Cannot create string")
            .to_string();

        let filter_id = create_body_filter(rule);

        if filter_id.is_none() {
            return null();
        }

        return str_to_cstr(filter_id.unwrap());
    }
}

#[wasm_bindgen]
pub fn body_filter(filter_id: String, filter_body: String) -> Option<String> {
    let has_filter: Option<filter::filter_body::FilterBodyAction> =
        FILTERS.lock().unwrap().remove(filter_id.as_str());

    if has_filter.is_none() {
        return None;
    }

    let mut filter = has_filter.unwrap();
    let result = filter.filter(filter_body);

    FILTERS.lock().unwrap().insert(filter_id, filter);

    return Some(result);
}

#[no_mangle]
pub extern "C" fn redirectionio_body_filter(
    filter_id_cstr: *const libc::c_char,
    filter_body_cstr: *const libc::c_char,
) -> *const libc::c_char {
    unsafe {
        let filter_id = std::ffi::CStr::from_ptr(filter_id_cstr)
            .to_str()
            .expect("Cannot create string")
            .to_string();
        let filter_body = std::ffi::CStr::from_ptr(filter_body_cstr)
            .to_str()
            .expect("Cannot create string")
            .to_string();

        let new_data = body_filter(filter_id, filter_body);

        if new_data.is_none() {
            return null();
        }

        return str_to_cstr(new_data.unwrap());
    }
}

#[wasm_bindgen]
pub extern "C" fn body_filter_end(filter_id: String) -> Option<String> {
    let has_filter: Option<filter::filter_body::FilterBodyAction> =
        FILTERS.lock().unwrap().remove(filter_id.as_str());

    if has_filter.is_none() {
        return None;
    }

    let mut filter = has_filter.unwrap();
    let result = filter.end();

    return Some(result);
}

#[no_mangle]
pub extern "C" fn redirectionio_body_filter_end(
    filter_id_cstr: *const libc::c_char,
) -> *const libc::c_char {
    unsafe {
        let filter_id = std::ffi::CStr::from_ptr(filter_id_cstr)
            .to_str()
            .expect("Cannot create string")
            .to_string();

        let new_data = body_filter_end(filter_id);

        if new_data.is_none() {
            return null();
        }

        return str_to_cstr(new_data.unwrap());
    }
}

pub fn str_to_cstr(str: String) -> *const libc::c_char {
    unsafe {
        let mut data: *const std::ffi::CString = 0 as *const std::ffi::CString;
        let boxed = Box::new(std::ffi::CString::new(str.as_bytes()).expect("Cannot create string"));

        data = transmute(boxed);

        return (&*data).as_ptr();
    };
}

fn rule_to_string(rule_obj: &router::rule::Rule) -> String {
    serde_json::to_string(rule_obj).expect("Cannot serialize rule")
}

fn string_to_rule(rule_str: String) -> Option<router::rule::Rule> {
    let rule_option: Option<router::rule::Rule> = serde_json::from_str(&rule_str).unwrap();

    if rule_option.is_none() {
        return None;
    }

    let mut rule = rule_option.unwrap();
    rule.compile(false);

    return Some(rule);
}
