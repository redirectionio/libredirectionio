use crate::router::Transform;

pub struct Lowercase {}

impl Transform for Lowercase {
    fn transform(&self, str: String) -> String {
        str.to_lowercase()
    }
}

impl Lowercase {
    pub fn new() -> Lowercase {
        Lowercase{}
    }
}
