use crate::router::Transform;
use heck::ToSnakeCase;

#[derive(Default)]
pub struct Underscorize;

impl Transform for Underscorize {
    fn transform(&self, str: String) -> String {
        str.to_snake_case()
    }
}
