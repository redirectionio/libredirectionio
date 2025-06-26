pub mod body_append;
pub mod body_prepend;
pub mod body_replace;

use std::{borrow::Cow, cell::RefCell, fmt::Debug, rc::Rc};

use lol_html::{ElementContentHandlers, Settings};

use crate::{
    action::UnitTrace,
    api::HTMLBodyFilter,
    filter::html_body_action::{body_append::BodyAppend, body_prepend::BodyPrepend, body_replace::BodyReplace},
};

#[derive(Debug)]
pub enum HtmlBodyVisitor {
    Append(BodyAppend),
    Prepend(BodyPrepend),
    Replace(BodyReplace),
}

impl HtmlBodyVisitor {
    pub fn new(filter: HTMLBodyFilter, unit_trace: Option<Rc<RefCell<UnitTrace>>>) -> Option<HtmlBodyVisitor> {
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
                unit_trace,
            ))),
            "prepend_child" => Some(HtmlBodyVisitor::Prepend(BodyPrepend::new(
                filter.element_tree,
                filter.css_selector,
                filter.value.clone(),
                filter.inner_value.unwrap_or(filter.value),
                filter.id,
                filter.target_hash,
                unit_trace,
            ))),
            "replace" => Some(HtmlBodyVisitor::Replace(BodyReplace::new(
                filter.element_tree,
                filter.css_selector,
                filter.value.clone(),
                filter.inner_value.unwrap_or(filter.value),
                filter.id,
                filter.target_hash,
                unit_trace,
            ))),
            _ => None,
        }
    }

    pub fn into_handlers(self, settings: &mut Settings) {
        match self {
            HtmlBodyVisitor::Append(append) => append.into_handlers(settings),
            HtmlBodyVisitor::Prepend(prepend) => {
                let Ok(selector) = prepend.css_selector().parse() else {
                    return;
                };

                settings
                    .element_content_handlers
                    .push((Cow::Owned(selector), ElementContentHandlers::default().element(prepend)));
            }
            HtmlBodyVisitor::Replace(replace) => {
                let Ok(selector) = replace.css_selector().parse() else {
                    return;
                };

                settings
                    .element_content_handlers
                    .push((Cow::Owned(selector), ElementContentHandlers::default().element(replace)));
            }
        }
    }
}
