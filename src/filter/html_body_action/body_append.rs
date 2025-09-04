use std::{
    borrow::Cow,
    cell::RefCell,
    rc::Rc,
    sync::{Arc, atomic::AtomicBool},
};

use lol_html::{
    ElementContentHandlers, Settings,
    html_content::{ContentType, Element},
};

use crate::{action::UnitTrace, filter::html_body_action::body_capture::CaptureRegistry};

#[derive(Debug)]
pub struct BodyAppend {
    element_tree: Vec<String>,
    css_selector: Option<String>,
    content: String,
    inner_content: String,
    id: Option<String>,
    target_hash: Option<String>,
    unit_trace: Option<Rc<RefCell<UnitTrace>>>,
    variables: Arc<CaptureRegistry>,
}

impl BodyAppend {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        element_tree: Vec<String>,
        css_selector: Option<String>,
        content: String,
        inner_content: String,
        id: Option<String>,
        target_hash: Option<String>,
        unit_trace: Option<Rc<RefCell<UnitTrace>>>,
        variables: Arc<CaptureRegistry>,
    ) -> BodyAppend {
        BodyAppend {
            element_tree,
            css_selector,
            content,
            inner_content,
            id,
            target_hash,
            unit_trace,
            variables,
        }
    }
}

impl BodyAppend {
    pub fn css_selector(&self) -> (String, Option<String>) {
        let mut element_tree = self.element_tree.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

        if let Some(css_selector) = &self.css_selector
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

    pub fn into_handlers(self, settings: &mut Settings) {
        let (base_css_selector, checker_selector) = match self.css_selector() {
            (base, Some(checker)) => match (checker.parse(), base.parse()) {
                (Ok(checker_selector), Ok(base_selector)) => (base_selector, Some(checker_selector)),
                _ => {
                    log::error!("Failed to parse CSS selector: {}", self.css_selector.as_deref().unwrap_or(""));
                    return;
                }
            },
            (base, None) => match base.parse() {
                Ok(selector) => (selector, None),
                Err(_) => {
                    log::error!("Failed to parse CSS selector: {}", self.css_selector.as_deref().unwrap_or(""));
                    return;
                }
            },
        };

        if checker_selector.is_none() {
            settings.element_content_handlers.push((
                Cow::Owned(base_css_selector),
                ElementContentHandlers::default().element(move |element: &mut Element| {
                    let content = self.variables.replace(self.content.clone());
                    element.append(content.as_str(), ContentType::Html);

                    if let (Some(unit_trace), Some(id)) = (self.unit_trace.clone(), &self.id) {
                        let inner_content = self.variables.replace(self.inner_content.clone());

                        unit_trace.borrow_mut().add_value_computed_by_unit(id, &inner_content);
                        if let Some(target_hash) = &self.target_hash {
                            unit_trace
                                .borrow_mut()
                                .override_unit_id_with_target(target_hash.as_str(), id.as_str());
                        } else {
                            unit_trace.borrow_mut().add_unit_id(id.clone());
                        }
                    }

                    Ok(())
                }),
            ));

            return;
        }

        let element_exists = Arc::new(AtomicBool::new(false));
        let element_exists_clone = Arc::clone(&element_exists);

        settings.element_content_handlers.push((
            Cow::Owned(checker_selector.unwrap()),
            ElementContentHandlers::default().element(move |_element: &mut Element| {
                element_exists.store(true, std::sync::atomic::Ordering::Relaxed);

                Ok(())
            }),
        ));

        settings.element_content_handlers.push((
            Cow::Owned(base_css_selector),
            ElementContentHandlers::default().element(move |element: &mut Element| {
                let element_exists_clone = Arc::clone(&element_exists_clone);
                let content = self.content.clone();
                let inner_content = self.inner_content.clone();
                let id = self.id.clone();
                let target_hash = self.target_hash.clone();
                let unit_trace_clone = self.unit_trace.clone();

                if let Some(handlers) = element.end_tag_handlers() {
                    handlers.push(Box::new(move |end| {
                        if element_exists_clone.load(std::sync::atomic::Ordering::Relaxed) {
                            return Ok(());
                        }

                        end.before(content.as_str(), ContentType::Html);

                        if let (Some(unit_trace), Some(id)) = (unit_trace_clone, &id) {
                            unit_trace.borrow_mut().add_value_computed_by_unit(id, &inner_content);
                            if let Some(target_hash) = &target_hash {
                                unit_trace
                                    .borrow_mut()
                                    .override_unit_id_with_target(target_hash.as_str(), id.as_str());
                            } else {
                                unit_trace.borrow_mut().add_unit_id(id.clone());
                            }
                        }

                        Ok(())
                    }));
                }

                Ok(())
            }),
        ));
    }
}
