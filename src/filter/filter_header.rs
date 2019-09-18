use crate::filter::header_action;
use crate::router::rule;

pub struct FilterHeaderAction {
    actions: Vec<Box<dyn header_action::HeaderAction>>,
}

impl FilterHeaderAction {
    pub fn new(rule_to_filter: rule::Rule) -> Option<FilterHeaderAction> {
        if rule_to_filter.header_filters.is_none() {
            return None;
        }

        let filters = rule_to_filter.header_filters.as_ref().unwrap();

        if filters.len() == 0 {
            return None;
        }

        let mut actions = Vec::new();

        for filter in filters {
            let action_filter = header_action::create_header_action(filter);

            if action_filter.is_some() {
                actions.push(action_filter.unwrap());
            }
        }

        return Some(FilterHeaderAction { actions });
    }

    pub fn filter(&self, mut headers: Vec<header_action::Header>) -> Vec<header_action::Header> {
        for filter in &self.actions {
            headers = filter.filter(headers);
        }

        return headers;
    }
}
