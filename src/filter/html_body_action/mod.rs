extern crate scraper;

pub mod body_append;
pub mod body_prepend;
pub mod body_replace;

use crate::action::UnitTrace;
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
                filter.value.clone(),
                filter.inner_value.unwrap_or(filter.value),
                filter.id,
                filter.target_hash,
            ))),
            "prepend_child" => Some(HtmlBodyVisitor::Prepend(BodyPrepend::new(
                filter.element_tree,
                filter.css_selector,
                filter.value.clone(),
                filter.inner_value.unwrap_or(filter.value),
                filter.id,
                filter.target_hash,
            ))),
            "replace" => Some(HtmlBodyVisitor::Replace(BodyReplace::new(
                filter.element_tree,
                filter.css_selector,
                filter.value.clone(),
                filter.inner_value.unwrap_or(filter.value),
                filter.id,
                filter.target_hash,
            ))),
            _ => None,
        }
    }

    pub fn enter(&mut self, data: String, unit_trace: Option<&mut UnitTrace>) -> (Option<String>, Option<String>, bool, String) {
        match self {
            Self::Append(append) => append.enter(data),
            Self::Prepend(prepend) => prepend.enter(data, unit_trace),
            Self::Replace(replace) => replace.enter(data),
        }
    }

    pub fn leave(&mut self, data: String, unit_trace: Option<&mut UnitTrace>) -> Result<(Option<String>, Option<String>, String)> {
        Ok(match self {
            Self::Append(append) => append.leave(data, unit_trace)?,
            Self::Prepend(prepend) => prepend.leave(data, unit_trace)?,
            Self::Replace(replace) => replace.leave(data, unit_trace),
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
    let selector = match scraper::Selector::parse(expression) {
        Ok(selector) => selector,
        Err(err) => {
            log::error!("cannot parse selector {}: {:?}", expression, err);

            return false;
        }
    };

    let document = scraper::Html::parse_fragment(data);
    let mut select = document.select(&selector);

    select.next().is_some()
}
