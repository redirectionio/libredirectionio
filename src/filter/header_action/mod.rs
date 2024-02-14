pub mod header_add;
pub mod header_default;
pub mod header_override;
pub mod header_remove;
pub mod header_replace;

use std::fmt::Debug;

use crate::{action::UnitTrace, api::HeaderFilter, http::Header};

pub trait HeaderAction: Debug + Send {
    fn filter(&self, headers: Vec<Header>, unit_trace: Option<&mut UnitTrace>) -> Vec<Header>;
}

pub fn create_header_action(header_filter: &HeaderFilter) -> Option<Box<dyn HeaderAction>> {
    if header_filter.action == "add" {
        return Some(Box::new(header_add::HeaderAddAction {
            id: header_filter.id.clone(),
            name: header_filter.header.clone(),
            value: header_filter.value.clone(),
            target_hash: header_filter.target_hash.clone(),
        }));
    }

    if header_filter.action == "remove" {
        return Some(Box::new(header_remove::HeaderRemoveAction {
            id: header_filter.id.clone(),
            name: header_filter.header.clone(),
            target_hash: header_filter.target_hash.clone(),
        }));
    }

    if header_filter.action == "replace" {
        return Some(Box::new(header_replace::HeaderReplaceAction {
            id: header_filter.id.clone(),
            name: header_filter.header.clone(),
            value: header_filter.value.clone(),
            target_hash: header_filter.target_hash.clone(),
        }));
    }

    if header_filter.action == "override" {
        return Some(Box::new(header_override::HeaderOverrideAction {
            id: header_filter.id.clone(),
            name: header_filter.header.clone(),
            value: header_filter.value.clone(),
            target_hash: header_filter.target_hash.clone(),
        }));
    }

    if header_filter.action == "default" {
        return Some(Box::new(header_default::HeaderDefaultAction {
            id: header_filter.id.clone(),
            name: header_filter.header.clone(),
            value: header_filter.value.clone(),
            target_hash: header_filter.target_hash.clone(),
        }));
    }

    None
}
