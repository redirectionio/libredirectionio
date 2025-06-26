use std::{cell::RefCell, rc::Rc};

use lol_html::{ElementHandler, html_content::ContentType, send::IntoHandler};

use crate::action::UnitTrace;

#[derive(Debug)]
pub struct BodyReplace {
    element_tree: Vec<String>,
    css_selector: Option<String>,
    content: String,
    inner_content: String,
    id: Option<String>,
    target_hash: Option<String>,
    unit_trace: Option<Rc<RefCell<UnitTrace>>>,
}

impl<'h> IntoHandler<ElementHandler<'h>> for BodyReplace {
    fn into_handler(self) -> ElementHandler<'h> {
        Box::new(move |element| {
            element.replace(self.content.as_str(), ContentType::Html);

            if let (Some(unit_trace), Some(id)) = (self.unit_trace.clone(), &self.id) {
                unit_trace.borrow_mut().add_value_computed_by_unit(id, &self.inner_content);
                if let Some(target_hash) = &self.target_hash {
                    unit_trace
                        .borrow_mut()
                        .override_unit_id_with_target(target_hash.as_str(), id.as_str());
                } else {
                    unit_trace.borrow_mut().add_unit_id(id.clone());
                }
            }

            Ok(())
        })
    }
}

impl BodyReplace {
    pub fn new(
        element_tree: Vec<String>,
        css_selector: Option<String>,
        content: String,
        inner_content: String,
        id: Option<String>,
        target_hash: Option<String>,
        unit_trace: Option<Rc<RefCell<UnitTrace>>>,
    ) -> BodyReplace {
        BodyReplace {
            element_tree,
            css_selector,
            content,
            inner_content,
            id,
            target_hash,
            unit_trace,
        }
    }
}

impl BodyReplace {
    pub fn css_selector(&self) -> String {
        let mut element_tree = self.element_tree.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

        if let Some(css_selector) = &self.css_selector {
            if !css_selector.is_empty() {
                if let Some(last) = element_tree.last() {
                    if css_selector.starts_with(last) {
                        element_tree.remove(element_tree.len() - 1);
                    }
                }

                return format!("{} > {}", element_tree.join(" > "), css_selector);
            }
        }

        element_tree.join(" > ")
    }
}
