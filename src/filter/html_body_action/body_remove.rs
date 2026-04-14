use std::{cell::RefCell, rc::Rc};

use lol_html::{ElementHandler, send::IntoHandler};

use crate::action::UnitTrace;

#[derive(Debug)]
pub struct BodyRemove {
    css_selector: String,
    id: Option<String>,
    target_hash: Option<String>,
    unit_trace: Option<Rc<RefCell<UnitTrace>>>,
}

impl<'h> IntoHandler<ElementHandler<'h>> for BodyRemove {
    fn into_handler(self) -> ElementHandler<'h> {
        Box::new(move |element| {
            element.remove();

            println!("BodyRemove: removed element with selector {}", self.css_selector);

            if let (Some(unit_trace), Some(id)) = (self.unit_trace.clone(), &self.id) {
                unit_trace.borrow_mut().add_value_computed_by_unit(id, "");

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

impl BodyRemove {
    pub fn new(
        css_selector: String,
        id: Option<String>,
        target_hash: Option<String>,
        unit_trace: Option<Rc<RefCell<UnitTrace>>>,
    ) -> BodyRemove {
        BodyRemove {
            css_selector,
            id,
            target_hash,
            unit_trace,
        }
    }
}

impl BodyRemove {
    pub fn css_selector(&self) -> String {
        self.css_selector.clone()
    }
}
