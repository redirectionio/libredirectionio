use crate::filter::body_action;

#[derive(Debug)]
pub struct BodyReplace {
    element_tree: Vec<String>,
    position: usize,
    x_path_matcher: Option<String>,
    content: String,
    is_buffering: bool,
}

impl BodyReplace {
    pub fn new(
        element_tree: Vec<String>,
        x_path_matcher: Option<String>,
        content: String,
    ) -> BodyReplace {
        BodyReplace {
            element_tree,
            x_path_matcher,
            position: 0,
            content,
            is_buffering: false,
        }
    }
}

impl body_action::BodyAction for BodyReplace {
    fn enter(&mut self, data: String) -> (Option<String>, Option<String>, bool, String) {
        let next_leave = Some(self.element_tree[self.position].clone());
        let mut next_enter = None;

        if self.position + 1 < self.element_tree.len() {
            self.position = self.position + 1;
            next_enter = Some(self.element_tree[self.position].clone());

            return (next_enter, next_leave, false, data);
        }

        if self.position + 1 >= self.element_tree.len() {
            self.is_buffering = true;

            return (next_enter, next_leave, true, data);
        }

        return (next_enter, next_leave, false, data);
    }

    fn leave(&mut self, data: String) -> (Option<String>, Option<String>, String) {
        let next_enter = Some(self.element_tree[self.position].clone());
        let mut next_leave = None;

        if self.position as i32 - 1 >= 0 {
            self.position = self.position - 1;

            next_leave = Some(self.element_tree[self.position].clone());
        }

        if self.is_buffering {
            self.is_buffering = false;

            if self.x_path_matcher.is_none() || self.x_path_matcher.as_ref().unwrap().is_empty() {
                return (next_enter, next_leave, self.content.clone());
            }

            if body_action::evaluate(data.clone(), self.x_path_matcher.as_ref().unwrap().clone()) {
                return (next_enter, next_leave, self.content.clone());
            }
        }

        return (next_enter, next_leave, data);
    }

    fn first(&self) -> String {
        return self.element_tree[0].clone();
    }
}
