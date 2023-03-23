use crate::marker::Transform;

#[derive(Default)]
pub struct Lowercase;

impl Transform for Lowercase {
    fn transform(&self, str: String) -> String {
        str.to_lowercase()
    }
}
