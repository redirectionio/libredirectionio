use std::{cell::RefCell, rc::Rc};

use crate::{action::UnitTrace, filter::header_action::HeaderAction, http::Header};

#[derive(Debug)]
pub struct HeaderOverrideAction {
    pub name: String,
    pub value: String,
    // in 3.0 make this mandatory
    pub id: Option<String>,
    // in 3.0 make this mandatory
    pub target_hash: Option<String>,
}

// Replace or add a header
impl HeaderAction for HeaderOverrideAction {
    fn filter(&self, headers: Vec<Header>, unit_trace: Option<Rc<RefCell<UnitTrace>>>) -> Vec<Header> {
        let mut new_headers = Vec::new();
        let mut found = false;

        for header in headers {
            if header.name.to_lowercase() != self.name.to_lowercase() {
                new_headers.push(header);
            } else {
                found = true;
                new_headers.push(Header {
                    name: self.name.clone(),
                    value: self.value.clone(),
                });
            }
        }

        if !found {
            new_headers.push(Header {
                name: self.name.clone(),
                value: self.value.clone(),
            });
        }

        if let (Some(trace), Some(id)) = (unit_trace, &self.id) {
            trace.borrow_mut().add_value_computed_by_unit(id, &self.value);

            if let Some(target_hash) = &self.target_hash {
                trace.borrow_mut().override_unit_id_with_target(target_hash, id);
            }
        }

        new_headers
    }
}
