use crate::router::Transform;

pub struct Slice {
    from: usize,
    to: Option<usize>,
}

impl Transform for Slice {
    fn transform(&self, str: String) -> String {
        let from = self.from;
        let mut to = self.to.unwrap_or(str.len());

        if from > str.len() {
            return "".to_string();
        }

        if to > str.len() {
            to = str.len();
        }

        str[from..to].to_string()
    }
}

impl Slice {
    pub fn new(from: usize, to: Option<usize>) -> Slice {
        Slice { from, to }
    }
}
