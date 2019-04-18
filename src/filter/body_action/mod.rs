pub mod body_append;
pub mod body_prepend;
pub mod body_replace;

use crate::router::rule;

pub trait BodyAction {
    fn enter(&mut self, data: String) -> (Option<String>, Option<String>, bool, String);
    fn leave(&mut self, data: String) -> (Option<String>, Option<String>, String);
}

//pub fn create_body_action(filter: rule::BodyFilter) -> Box<BodyAction> {}
