extern crate cfg_if;
extern crate wasm_bindgen;
#[macro_use]
extern crate lazy_static;

mod api;
mod filter;
mod html;
mod router;
mod utils;

use cfg_if::cfg_if;
use std::collections::HashMap;
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

#[wasm_bindgen]
pub fn get_redirect(rule_str: String, url: String) -> Option<String> {
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
