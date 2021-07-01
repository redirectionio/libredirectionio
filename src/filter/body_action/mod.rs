extern crate scraper;

pub mod body_append;
pub mod body_prepend;
pub mod body_replace;

use crate::api::BodyFilter;
use std::fmt::Debug;

pub trait BodyAction: Debug + Send {
    fn enter(&mut self, data: String) -> (Option<String>, Option<String>, bool, String);
    fn leave(&mut self, data: String) -> (Option<String>, Option<String>, String);
    fn first(&self) -> String;
}

pub fn evaluate(data: String, expression: String) -> bool {
    let document = scraper::Html::parse_fragment(data.as_str());
    let selector_result = scraper::Selector::parse(expression.as_str());

    if selector_result.is_err() {
        error!("Cannot parse selector {}: {:?}", expression, selector_result.err().unwrap());

        return false;
    }

    let selector = selector_result.unwrap();
    let mut select = document.select(&selector);

    select.next().is_some()
}

pub fn create_body_action(filter: &BodyFilter) -> Option<Box<dyn BodyAction>> {
    if filter.element_tree.is_empty() {
        return None;
    }

    if filter.action == "append_child" {
        return Some(Box::new(body_append::BodyAppend::new(
            filter.element_tree.clone(),
            filter.css_selector.clone(),
            filter.value.clone(),
        )));
    }

    if filter.action == "prepend_child" {
        return Some(Box::new(body_prepend::BodyPrepend::new(
            filter.element_tree.clone(),
            filter.css_selector.clone(),
            filter.value.clone(),
        )));
    }

    if filter.action == "replace" {
        return Some(Box::new(body_replace::BodyReplace::new(
            filter.element_tree.clone(),
            filter.css_selector.clone(),
            filter.value.clone(),
        )));
    }

    None
}
