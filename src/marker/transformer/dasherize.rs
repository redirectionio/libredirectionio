use heck::ToKebabCase;

use crate::marker::Transform;

#[derive(Default)]
pub struct Dasherize;

impl Transform for Dasherize {
    fn transform(&self, str: String) -> String {
        str.to_kebab_case()
    }
}
