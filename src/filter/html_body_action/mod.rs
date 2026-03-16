pub mod body_after;
pub mod body_append;
pub mod body_before;
pub mod body_capture;
pub mod body_prepend;
pub mod body_remove;
pub mod body_replace;

use std::{borrow::Cow, cell::RefCell, fmt::Debug, rc::Rc, sync::Arc};

use lol_html::{ElementContentHandlers, Settings};

use crate::{
    action::UnitTrace,
    api::HTMLBodyFilter,
    filter::html_body_action::{
        body_after::BodyAfter,
        body_append::BodyAppend,
        body_before::BodyBefore,
        body_capture::{BodyCapture, CaptureRegistry},
        body_prepend::BodyPrepend,
        body_remove::BodyRemove,
        body_replace::BodyReplace,
    },
};

#[derive(Debug)]
pub enum HtmlBodyVisitor {
    Append(BodyAppend),
    Prepend(BodyPrepend),
    Replace(BodyReplace),
    Capture(BodyCapture),
    Remove(BodyRemove),
    After(BodyAfter),
    Before(BodyBefore),
}

impl HtmlBodyVisitor {
    pub fn new(
        filter: HTMLBodyFilter,
        unit_trace: Option<Rc<RefCell<UnitTrace>>>,
        variables: Arc<CaptureRegistry>,
    ) -> Option<HtmlBodyVisitor> {
        match filter {
            HTMLBodyFilter::PrependLegacy(filter) => {
                let css_selector = build_css_selector(&filter.element_tree, &filter.css_selector);

                if css_selector.is_empty() {
                    return None;
                }

                Some(HtmlBodyVisitor::Prepend(BodyPrepend::new(
                    css_selector,
                    filter.value.clone(),
                    filter.inner_value.unwrap_or(filter.value),
                    filter.id,
                    filter.target_hash,
                    unit_trace,
                    variables.clone(),
                )))
            }
            HTMLBodyFilter::AppendLegacy(filter) => {
                let (css_selector, css_selector_ignore) = build_append_css_selector(&filter.element_tree, &filter.css_selector);

                Some(HtmlBodyVisitor::Append(BodyAppend::new(
                    css_selector,
                    css_selector_ignore,
                    filter.value.clone(),
                    filter.inner_value.unwrap_or(filter.value),
                    filter.id,
                    filter.target_hash,
                    unit_trace,
                    variables.clone(),
                )))
            }
            HTMLBodyFilter::ReplaceLegacy(filter) => {
                let css_selector = build_css_selector(&filter.element_tree, &filter.css_selector);

                if css_selector.is_empty() {
                    return None;
                }

                Some(HtmlBodyVisitor::Replace(BodyReplace::new(
                    css_selector,
                    filter.value.clone(),
                    filter.inner_value.unwrap_or(filter.value),
                    filter.id,
                    filter.target_hash,
                    unit_trace,
                    variables.clone(),
                )))
            }
            HTMLBodyFilter::Append(filter) => Some(HtmlBodyVisitor::Append(BodyAppend::new(
                filter.css_selector,
                filter.ignore_css_selector,
                filter.value.clone(),
                filter.inner_value.unwrap_or(filter.value),
                filter.id,
                filter.target_hash,
                unit_trace,
                variables.clone(),
            ))),
            HTMLBodyFilter::Prepend(filter) => Some(HtmlBodyVisitor::Prepend(BodyPrepend::new(
                filter.css_selector,
                filter.value.clone(),
                filter.inner_value.unwrap_or(filter.value),
                filter.id,
                filter.target_hash,
                unit_trace,
                variables.clone(),
            ))),
            HTMLBodyFilter::Replace(filter) => Some(HtmlBodyVisitor::Replace(BodyReplace::new(
                filter.css_selector,
                filter.value.clone(),
                filter.inner_value.unwrap_or(filter.value),
                filter.id,
                filter.target_hash,
                unit_trace,
                variables.clone(),
            ))),
            HTMLBodyFilter::After(filter) => Some(HtmlBodyVisitor::After(BodyAfter::new(
                filter.css_selector,
                filter.value.clone(),
                filter.inner_value.unwrap_or(filter.value),
                filter.id,
                filter.target_hash,
                unit_trace,
                variables.clone(),
            ))),
            HTMLBodyFilter::Before(filter) => Some(HtmlBodyVisitor::Before(BodyBefore::new(
                filter.css_selector,
                filter.value.clone(),
                filter.inner_value.unwrap_or(filter.value),
                filter.id,
                filter.target_hash,
                unit_trace,
                variables.clone(),
            ))),
            HTMLBodyFilter::Remove(filter) => Some(HtmlBodyVisitor::Remove(BodyRemove::new(
                filter.css_selector,
                filter.id,
                filter.target_hash,
                unit_trace,
            ))),
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
            HtmlBodyVisitor::Capture(capture) => {
                capture.into_handlers(settings);
            }
            HtmlBodyVisitor::Remove(remove) => {
                let Ok(selector) = remove.css_selector().parse() else {
                    return;
                };

                settings
                    .element_content_handlers
                    .push((Cow::Owned(selector), ElementContentHandlers::default().element(remove)));
            }
            HtmlBodyVisitor::After(after) => {
                let Ok(selector) = after.css_selector().parse() else {
                    return;
                };

                settings
                    .element_content_handlers
                    .push((Cow::Owned(selector), ElementContentHandlers::default().element(after)));
            }
            HtmlBodyVisitor::Before(before) => {
                let Ok(selector) = before.css_selector().parse() else {
                    return;
                };

                settings
                    .element_content_handlers
                    .push((Cow::Owned(selector), ElementContentHandlers::default().element(before)));
            }
        }
    }
}

fn build_css_selector(element_tree: &[String], css_selector: &Option<String>) -> String {
    let mut element_tree = element_tree.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    if let Some(css_selector) = &css_selector
        && !css_selector.is_empty()
    {
        if element_tree.is_empty() {
            return css_selector.clone();
        }

        if let Some(last) = element_tree.last()
            && css_selector.starts_with(last)
        {
            element_tree.remove(element_tree.len() - 1);
        }

        return format!("{} > {}", element_tree.join(" > "), css_selector);
    }

    element_tree.join(" > ")
}

fn build_append_css_selector(element_tree: &[String], css_selector: &Option<String>) -> (String, Option<String>) {
    let mut element_tree = element_tree.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    if let Some(css_selector) = &css_selector
        && !css_selector.is_empty()
    {
        if let Some(last) = element_tree.last()
            && css_selector.starts_with(last)
        {
            element_tree.remove(element_tree.len() - 1);
        }

        let base_css_selector = element_tree.join(" > ");
        let checker_selector = format!("{} > {}", element_tree.join(" > "), css_selector);

        return (base_css_selector, Some(checker_selector));
    }

    (element_tree.join(" > "), None)
}
