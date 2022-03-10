use crate::router::Transform;
use heck::ToLowerCamelCase;

#[derive(Default)]
pub struct Camelize;

impl Transform for Camelize {
    fn transform(&self, str: String) -> String {
        str.to_lower_camel_case()
    }
}
