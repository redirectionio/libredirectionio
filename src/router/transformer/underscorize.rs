use crate::router::Transform;
use heck::SnakeCase;

pub struct Underscorize {}

impl Transform for Underscorize {
    fn transform(&self, str: String) -> String {
        str.to_snake_case()
    }
}

impl Underscorize {
    pub fn new() -> Underscorize {
        Underscorize{}
    }
}
