extern crate scraper;

pub mod body_append;
pub mod body_prepend;
pub mod body_replace;

use crate::api::HTMLBodyFilter;
use crate::filter::error::Result;
use crate::filter::html_body_action::body_append::BodyAppend;
use crate::filter::html_body_action::body_prepend::BodyPrepend;
use crate::filter::html_body_action::body_replace::BodyReplace;
use std::fmt::Debug;

#[derive(Debug)]
pub enum HtmlBodyVisitor {
    Append(BodyAppend),
    Prepend(BodyPrepend),
    Replace(BodyReplace),
}

impl HtmlBodyVisitor {
    pub fn new(filter: HTMLBodyFilter) -> Option<HtmlBodyVisitor> {
        if filter.element_tree.is_empty() {
            return None;
        }

        match filter.action.as_str() {
            "append_child" => Some(HtmlBodyVisitor::Append(BodyAppend::new(
                filter.element_tree,
                filter.css_selector,
                filter.value,
            ))),
            "prepend_child" => Some(HtmlBodyVisitor::Prepend(BodyPrepend::new(
                filter.element_tree,
                filter.css_selector,
                filter.value,
            ))),
            "replace" => Some(HtmlBodyVisitor::Replace(BodyReplace::new(
                filter.element_tree,
                filter.css_selector,
                filter.value,
            ))),
            _ => None,
        }
    }

    pub fn enter(&mut self, data: String) -> (Option<String>, Option<String>, bool, String) {
        match self {
            Self::Append(append) => append.enter(data),
            Self::Prepend(prepend) => prepend.enter(data),
            Self::Replace(replace) => replace.enter(data),
        }
    }

    pub fn leave(&mut self, data: String) -> Result<(Option<String>, Option<String>, String)> {
        Ok(match self {
            Self::Append(append) => append.leave(data)?,
            Self::Prepend(prepend) => prepend.leave(data)?,
            Self::Replace(replace) => replace.leave(data),
        })
    }

    pub fn first(&self) -> String {
        match self {
            Self::Append(append) => append.first(),
            Self::Prepend(prepend) => prepend.first(),
            Self::Replace(replace) => replace.first(),
        }
    }
}

pub fn evaluate(data: &str, expression: &str) -> bool {
    let document = scraper::Html::parse_fragment(data);
    let selector_result = scraper::Selector::parse(expression);

    if selector_result.is_err() {
        error!("Cannot parse selector {}: {:?}", expression, selector_result.err().unwrap());

        return false;
    }

    let selector = selector_result.unwrap();
    let mut select = document.select(&selector);

    select.next().is_some()
}
