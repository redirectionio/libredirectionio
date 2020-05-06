pub mod header_add;
pub mod header_override;
pub mod header_remove;
pub mod header_replace;

use crate::api::HeaderFilter;
use crate::http::Header;
use std::fmt::Debug;

pub trait HeaderAction: Debug + Send {
    fn filter(&self, headers: Vec<Header>) -> Vec<Header>;
}

pub fn create_header_action(header_filter: &HeaderFilter) -> Option<Box<dyn HeaderAction>> {
    if header_filter.action == "add" {
        return Some(Box::new(header_add::HeaderAddAction {
            name: header_filter.header.clone(),
            value: header_filter.value.clone(),
        }));
    }

    if header_filter.action == "remove" {
        return Some(Box::new(header_remove::HeaderRemoveAction {
            name: header_filter.header.clone(),
        }));
    }

    if header_filter.action == "replace" {
        return Some(Box::new(header_replace::HeaderReplaceAction {
            name: header_filter.header.clone(),
            value: header_filter.value.clone(),
        }));
    }

    if header_filter.action == "override" {
        return Some(Box::new(header_override::HeaderOverrideAction {
            name: header_filter.header.clone(),
            value: header_filter.value.clone(),
        }));
    }

    None
}
