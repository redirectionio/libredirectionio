use crate::router::Transform;
use heck::ToKebabCase;

#[derive(Default)]
pub struct Dasherize;

impl Transform for Dasherize {
    fn transform(&self, str: String) -> String {
        str.to_kebab_case()
    }
}
