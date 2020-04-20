use crate::router::Transform;

pub struct Uppercase {}

impl Transform for Uppercase {
    fn transform(&self, str: String) -> String {
        str.to_uppercase()
    }
}

impl Uppercase {
    pub fn new() -> Uppercase {
        Uppercase{}
    }
}
