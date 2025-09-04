use std::{cell::RefCell, rc::Rc, sync::Arc};

use lol_html::{ElementHandler, html_content::ContentType, send::IntoHandler};

use crate::{action::UnitTrace, filter::html_body_action::body_capture::CaptureRegistry};

#[derive(Debug)]
pub struct BodyPrepend {
    element_tree: Vec<String>,
    css_selector: Option<String>,
    content: String,
    inner_content: String,
    id: Option<String>,
    target_hash: Option<String>,
    unit_trace: Option<Rc<RefCell<UnitTrace>>>,
    variables: Arc<CaptureRegistry>,
}

impl<'h> IntoHandler<ElementHandler<'h>> for BodyPrepend {
    fn into_handler(self) -> ElementHandler<'h> {
        Box::new(move |element| {
            let content = self.variables.replace(self.content.clone());
            element.prepend(content.as_str(), ContentType::Html);

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
        })
    }
}

impl BodyPrepend {
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
    ) -> BodyPrepend {
        BodyPrepend {
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

impl BodyPrepend {
    pub fn css_selector(&self) -> String {
        let mut element_tree = self.element_tree.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

        if let Some(css_selector) = &self.css_selector
            && !css_selector.is_empty()
        {
            if let Some(last) = element_tree.last()
                && css_selector.starts_with(last)
            {
                element_tree.remove(element_tree.len() - 1);
            }

            return format!("{} > {}", element_tree.join(" > "), css_selector);
        }

        element_tree.join(" > ")
    }
}
