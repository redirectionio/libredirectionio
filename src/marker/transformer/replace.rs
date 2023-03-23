use crate::marker::Transform;

pub struct Replace {
    something: String,
    with: String,
}

impl Transform for Replace {
    fn transform(&self, str: String) -> String {
        str.replace(self.something.as_str(), self.with.as_str())
    }
}

impl Replace {
    pub fn new(something: String, with: String) -> Replace {
        Replace { something, with }
    }
}
