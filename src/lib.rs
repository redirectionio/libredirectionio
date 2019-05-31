extern crate cfg_if;
extern crate wasm_bindgen;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate log;
extern crate stderrlog;

#[cfg(not(target_arch = "wasm32"))]
pub mod callback_log;
mod filter;
pub mod html;
mod router;
mod utils;

use cfg_if::cfg_if;
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::intrinsics::transmute;
#[cfg(not(target_arch = "wasm32"))]
use std::ptr::null;
use std::sync::Mutex;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::{Once, ONCE_INIT};
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

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        fn init_log() {
            console_log::init_with_level(log::Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {
            stderrlog::new().module(module_path!()).init().unwrap();
        }
    }
}

lazy_static! {
    static ref PROJECT_ROUTERS: Mutex<HashMap<String, router::MainRouter>> =
        Mutex::new(HashMap::new());
    static ref FILTERS: Mutex<HashMap<String, filter::filter_body::FilterBodyAction>> =
        Mutex::new(HashMap::new());
}

#[cfg(not(target_arch = "wasm32"))]
static mut LOGGER: callback_log::CallbackLogger = callback_log::CallbackLogger {
    callback: None,
    data: None,
};

#[cfg(not(target_arch = "wasm32"))]
static INIT: Once = ONCE_INIT;

#[wasm_bindgen]
#[no_mangle]
pub extern "C" fn redirectionio_init_log() {
    init_log();
}

#[no_mangle]
#[cfg(not(target_arch = "wasm32"))]
pub extern "C" fn redirectionio_init_log_callback(
    callback: callback_log::redirectionio_log_callback,
    data: &'static libc::c_void,
) {
    unsafe {
        LOGGER.callback = Some(callback);
        LOGGER.data = Some(data);

        INIT.call_once(|| {
            log::set_logger(&LOGGER)
                .map(|()| log::set_max_level(log::LevelFilter::Trace))
                .expect("cannot set logger");
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
macro_rules! cstr_to_str {
    ($cstr:expr, $str:ident, $origin:expr) => {
        let cstring = std::ffi::CStr::from_ptr($cstr);
        let result = cstring.to_str();

        if result.is_err() {
            error!("Unable to create string for {} '{}': {}", $origin, String::from_utf8_lossy(cstring.to_bytes()), result.err().unwrap());

            return null();
        }

        let $str = result.unwrap().to_string();
    };
}

#[wasm_bindgen]
pub fn update_rules_for_router(project_id: String, rules_data: String, cache: bool) -> String {
    utils::set_panic_hook();
    let main_router_result = router::MainRouter::new_from_data(rules_data, cache);

    if main_router_result.is_err() {
        error!(
            "Cannot create router: {}",
            main_router_result.err().unwrap()
        );

        return project_id;
    }

    PROJECT_ROUTERS
        .lock()
        .unwrap()
        .insert(project_id.clone(), main_router_result.unwrap());

    return project_id;
}

#[no_mangle]
#[cfg(not(target_arch = "wasm32"))]
pub extern "C" fn redirectionio_update_rules_for_router(
    project_id_cstr: *const libc::c_char,
    rules_data_cstr: *const libc::c_char,
    cache: libc::c_uint,
) -> *const libc::c_char {
    unsafe {
        cstr_to_str!(project_id_cstr, project_id, "project id");
        cstr_to_str!(rules_data_cstr, rules_data, "rules data");

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

    let rule_result = router.unwrap().match_rule(url.clone());

    if rule_result.is_err() {
        error!(
            "Cannot match rule for url {}: {}",
            url,
            rule_result.err().unwrap()
        );

        return None;
    }

    let rule = rule_result.unwrap();

    if rule.is_none() {
        return None;
    }

    rule_to_string(rule.unwrap())
}

#[no_mangle]
#[cfg(not(target_arch = "wasm32"))]
pub extern "C" fn redirectionio_get_rule_for_url(
    project_id_cstr: *const libc::c_char,
    url_cstr: *const libc::c_char,
) -> *const libc::c_char {
    unsafe {
        cstr_to_str!(project_id_cstr, project_id, "project id in get rule");
        cstr_to_str!(url_cstr, url, "url in get rule");

        let rule_data = get_rule_for_url(project_id, url);

        if rule_data.is_none() {
            return null();
        }

        return str_to_cstr(rule_data.unwrap());
    }
}

#[wasm_bindgen]
pub fn get_trace_for_url(project_id: String, url: String) -> Option<String> {
    let lock = PROJECT_ROUTERS.lock();
    let router: Option<&router::MainRouter> = lock.as_ref().unwrap().get(project_id.as_str());

    if router.is_none() {
        return None;
    }

    let trace_result = router.unwrap().trace(url.clone());

    if trace_result.is_err() {
        error!("Cannot trace url {}: {}", url, trace_result.err().unwrap());

        return None;
    }

    let trace = trace_result.unwrap();
    let trace_str_result = serde_json::to_string(&trace);

    if trace_str_result.is_err() {
        error!(
            "Cannot serialize trace {:?}: {}",
            trace,
            trace_str_result.err().unwrap()
        );

        return None;
    }

    return Some(trace_str_result.unwrap());
}

#[no_mangle]
#[cfg(not(target_arch = "wasm32"))]
pub extern "C" fn redirectionio_get_trace_for_url(
    project_id_cstr: *const libc::c_char,
    url_cstr: *const libc::c_char,
) -> *const libc::c_char {
    unsafe {
        cstr_to_str!(project_id_cstr, project_id, "project id in get trace");
        cstr_to_str!(url_cstr, url, "url in get trace");

        let trace_data = get_trace_for_url(project_id, url);

        if trace_data.is_none() {
            return null();
        }

        return str_to_cstr(trace_data.unwrap());
    }
}

#[wasm_bindgen]
pub fn get_redirect(rule_str: String, url: String, response_code: u16) -> Option<String> {
    if rule_str.is_empty() {
        return None;
    }

    let rule = string_to_rule(rule_str);

    if rule.is_none() {
        return None;
    }

    let rule_obj = rule.unwrap();

    if rule_obj.id.is_empty() {
        return None;
    }

    if rule_obj.redirect_code == 0 {
        return None;
    }

    if rule_obj.match_on_response_status.is_some()
        && rule_obj.match_on_response_status.unwrap() != response_code
    {
        return None;
    }

    let target_result = router::MainRouter::get_redirect(&rule_obj, url.clone());

    if target_result.is_err() {
        error!(
            "Cannot create target for rule {:?} on url {}: {}",
            rule_obj,
            url,
            target_result.err().unwrap()
        );

        return None;
    }

    let target = target_result.unwrap();
    let redirect = router::rule::Redirect {
        status: rule_obj.redirect_code,
        target,
    };

    let redirect_str_result = serde_json::to_string(&redirect);

    if redirect_str_result.is_err() {
        error!(
            "Cannot serialize redirect {:?}: {}",
            redirect,
            redirect_str_result.err().unwrap()
        );

        return None;
    }

    return Some(redirect_str_result.unwrap());
}

#[no_mangle]
#[cfg(not(target_arch = "wasm32"))]
pub extern "C" fn redirectionio_get_redirect(
    rule_cstr: *const libc::c_char,
    url_cstr: *const libc::c_char,
    response_code: libc::uint16_t,
) -> *const libc::c_char {
    unsafe {
        cstr_to_str!(rule_cstr, rule, "rule in get redirect");
        cstr_to_str!(url_cstr, url, "url in get redirect");

        let redirect = get_redirect(rule, url, response_code);

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

    let new_headers_str_result = serde_json::to_string(&new_headers);

    if new_headers_str_result.is_err() {
        error!(
            "Cannot serializer new headers {:?}: {}",
            new_headers,
            new_headers_str_result.err().unwrap()
        );

        return headers_str;
    }

    return new_headers_str_result.unwrap();
}

#[no_mangle]
#[cfg(not(target_arch = "wasm32"))]
pub extern "C" fn redirectionio_header_filter(
    rule_cstr: *const libc::c_char,
    headers_cstr: *const libc::c_char,
) -> *const libc::c_char {
    unsafe {
        cstr_to_str!(rule_cstr, rule, "rule in header filtering");
        cstr_to_str!(headers_cstr, headers, "headers in header filtering");

        let new_headers_str = header_filter(rule, headers);

        return str_to_cstr(new_headers_str);
    }
}

#[wasm_bindgen]
pub fn create_body_filter(rule_str: String, filter_id: String) -> Option<String> {
    let rule = string_to_rule(rule_str);

    if rule.is_none() {
        return None;
    }

    let rule_obj = rule.unwrap();

    let filter = filter::filter_body::FilterBodyAction::new(rule_obj);

    if filter.is_none() {
        return None;
    }

    let mut uuid = filter_id;

    if uuid.is_empty() {
        uuid = Uuid::new_v4().to_string();
    }

    FILTERS
        .lock()
        .unwrap()
        .insert(uuid.clone(), filter.unwrap());

    return Some(uuid);
}

#[no_mangle]
#[cfg(not(target_arch = "wasm32"))]
pub extern "C" fn redirectionio_create_body_filter(
    rule_cstr: *const libc::c_char,
) -> *const libc::c_char {
    unsafe {
        cstr_to_str!(rule_cstr, rule, "rule in create body filter");

        let filter_id = create_body_filter(rule, "".to_string());

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
#[cfg(not(target_arch = "wasm32"))]
pub extern "C" fn redirectionio_body_filter(
    filter_id_cstr: *const libc::c_char,
    filter_body_cstr: *const libc::c_char,
) -> *const libc::c_char {
    unsafe {
        cstr_to_str!(filter_id_cstr, filter_id, "filter id in body filtering");
        cstr_to_str!(filter_body_cstr, filter_body, "filter body in body filtering");

        let new_data = body_filter(filter_id, filter_body);

        if new_data.is_none() {
            return null();
        }

        return str_to_cstr(new_data.unwrap());
    }
}

#[wasm_bindgen]
pub fn body_filter_end(filter_id: String) -> Option<String> {
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
#[cfg(not(target_arch = "wasm32"))]
pub extern "C" fn redirectionio_body_filter_end(
    filter_id_cstr: *const libc::c_char,
) -> *const libc::c_char {
    unsafe {
        cstr_to_str!(filter_id_cstr, filter_id, "filter id in body filtering ending");

        let new_data = body_filter_end(filter_id);

        if new_data.is_none() {
            return null();
        }

        return str_to_cstr(new_data.unwrap());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn str_to_cstr(str: String) -> *const libc::c_char {
    unsafe {
        let string_result = std::ffi::CString::new(str.as_bytes());

        if string_result.is_err() {
            error!(
                "Cannot create c string {}: {}",
                str,
                string_result.err().unwrap()
            );

            return null();
        }

        let data: *const std::ffi::CString = transmute(Box::new(string_result.unwrap()));

        return (&*data).as_ptr();
    };
}

fn rule_to_string(rule_obj: &router::rule::Rule) -> Option<String> {
    let rule_result = serde_json::to_string(rule_obj);

    if rule_result.is_err() {
        error!(
            "Unable to create string from rule {:?}: {}",
            rule_obj,
            rule_result.err().unwrap()
        );

        return None;
    }

    return Some(rule_result.unwrap());
}

fn string_to_rule(rule_str: String) -> Option<router::rule::Rule> {
    let rule_result = serde_json::from_str(&rule_str);

    if rule_result.is_err() {
        error!(
            "Unable to create rule from string {}: {}",
            rule_str,
            rule_result.err().unwrap()
        );

        return None;
    }

    let rule_option: Option<router::rule::Rule> = rule_result.unwrap();

    if rule_option.is_none() {
        return None;
    }

    let mut rule = rule_option.unwrap();
    let compile_result = rule.compile(false);

    if compile_result.is_err() {
        error!(
            "Unable to compile rule {:?}: {}",
            rule,
            compile_result.err().unwrap()
        );

        return None;
    }

    return Some(rule);
}
