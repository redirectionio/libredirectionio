use crate::filter::header_action;
use crate::router::rule;

pub struct FilterHeaderAction {
    actions: Vec<Box<dyn header_action::HeaderAction>>,
}

impl FilterHeaderAction {
    pub fn new(rule_to_filter: rule::Rule) -> Option<FilterHeaderAction> {
        let filters = rule_to_filter.header_filters.as_ref()?;

        if filters.is_empty() {
            return None;
        }

        let mut actions = Vec::new();

        for filter in filters {
            if let Some(action_filter) = header_action::create_header_action(filter) {
                actions.push(action_filter);
            }
        }

        Some(FilterHeaderAction { actions })
    }

    pub fn filter(&self, mut headers: Vec<header_action::Header>) -> Vec<header_action::Header> {
        for filter in &self.actions {
            headers = filter.filter(headers);
        }

        headers
    }
}
