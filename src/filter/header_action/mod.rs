pub mod header_add;
pub mod header_remove;
pub mod header_replace;

use crate::router::rule;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl Header {
    pub fn new(name: String, value: String) -> Header {
        Header { name, value }
    }
}

pub trait HeaderAction: Debug + Send {
    fn filter(&self, headers: Vec<Header>) -> Vec<Header>;
}

pub fn create_header_action(header_filter: &rule::HeaderFilter) -> Option<Box<dyn HeaderAction>> {
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

    None
}
