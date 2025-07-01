use std::{cell::RefCell, rc::Rc};

use crate::{action::UnitTrace, api::HeaderFilter, filter::header_action, http::Header};

pub struct FilterHeaderAction {
    actions: Vec<Box<dyn header_action::HeaderAction>>,
}

impl FilterHeaderAction {
    pub fn new(filters: Vec<HeaderFilter>) -> Option<FilterHeaderAction> {
        if filters.is_empty() {
            return None;
        }

        let mut actions = Vec::new();

        for filter in &filters {
            if let Some(action_filter) = header_action::create_header_action(filter) {
                actions.push(action_filter);
            }
        }

        if actions.is_empty() {
            return None;
        }

        Some(FilterHeaderAction { actions })
    }

    pub fn filter(&self, mut headers: Vec<Header>, unit_trace: Option<Rc<RefCell<UnitTrace>>>) -> Vec<Header> {
        for filter in &self.actions {
            headers = filter.filter(headers, unit_trace.clone());
        }

        headers
    }
}
