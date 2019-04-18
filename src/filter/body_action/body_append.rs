use crate::filter::body_action;

struct BodyAppend {
    element_tree: Vec<String>,
    position: usize,
    x_path_matcher: Option<String>,
    content: String,
    is_buffering: bool,
}

impl BodyAppend {
    pub fn new(
        element_tree: Vec<String>,
        x_path_matcher: Option<String>,
        content: String,
    ) -> BodyAppend {
        BodyAppend {
            element_tree,
            x_path_matcher,
            position: 0,
            content,
            is_buffering: false,
        }
    }
}

impl body_action::BodyAction for BodyAppend {
    fn enter(&mut self, data: String) -> (Option<String>, Option<String>, bool, String) {
        let next_leave = Some(self.element_tree[self.position].clone());
        let mut next_enter = None;

        if self.position + 1 < self.element_tree.len() {
            self.position = self.position + 1;
            next_enter = Some(self.element_tree[self.position].clone());
        }

        let should_buffer =
            self.position + 1 >= self.element_tree.len() && self.x_path_matcher.is_some();

        return (next_enter, next_leave, should_buffer, data);
    }

    fn leave(&mut self, data: String) -> (Option<String>, Option<String>, String) {
        return (None, None, data);
    }
}
