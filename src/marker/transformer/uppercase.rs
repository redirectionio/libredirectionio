use crate::marker::Transform;

#[derive(Default)]
pub struct Uppercase;

impl Transform for Uppercase {
    fn transform(&self, str: String) -> String {
        str.to_uppercase()
    }
}
