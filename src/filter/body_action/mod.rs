extern crate sxd_document;
extern crate sxd_xpath;

use std::fmt::Debug;

pub mod body_append;
pub mod body_prepend;
pub mod body_replace;

pub trait BodyAction: Debug + Send {
    fn enter(&mut self, data: String) -> (Option<String>, Option<String>, bool, String);
    fn leave(&mut self, data: String) -> (Option<String>, Option<String>, String);
    fn first(&self) -> String;
}

use crate::router::rule;
use sxd_document::parser;
use sxd_xpath::evaluate_xpath;

pub fn evaluate(data: String, expression: String) -> bool {
    let package = parser::parse(data.as_str()).expect("failed to parse XML");
    let document = package.as_document();

    let value = evaluate_xpath(&document, expression.as_str()).expect("XPath evaluation failed");

    return value.boolean();
}

pub fn create_body_action(filter: &rule::BodyFilter) -> Option<Box<BodyAction>> {
    if filter.element_tree.len() == 0 {
        return None;
    }

    if filter.action == "append_child" {
        return Some(Box::new(body_append::BodyAppend::new(
            filter.element_tree.clone(),
            filter.x_path_matcher.clone(),
            filter.value.clone()
        )))
    }

    if filter.action == "prepend_child" {
        return Some(Box::new(body_prepend::BodyPrepend::new(
            filter.element_tree.clone(),
            filter.x_path_matcher.clone(),
            filter.value.clone()
        )))
    }

    if filter.action == "replace" {
        return Some(Box::new(body_replace::BodyReplace::new(
            filter.element_tree.clone(),
            filter.x_path_matcher.clone(),
            filter.value.clone()
        )))
    }

    return None;
}
