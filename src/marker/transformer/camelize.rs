use heck::ToLowerCamelCase;

use crate::marker::Transform;

#[derive(Default)]
pub struct Camelize;

impl Transform for Camelize {
    fn transform(&self, str: String) -> String {
        str.to_lower_camel_case()
    }
}
