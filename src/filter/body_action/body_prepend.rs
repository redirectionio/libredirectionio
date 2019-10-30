use crate::filter::body_action;
use crate::html;

#[derive(Debug)]
pub struct BodyPrepend {
    element_tree: Vec<String>,
    position: usize,
    css_selector: Option<String>,
    content: String,
    is_buffering: bool,
}

impl BodyPrepend {
    pub fn new(
        element_tree: Vec<String>,
        css_selector: Option<String>,
        content: String,
    ) -> BodyPrepend {
        BodyPrepend {
            element_tree,
            css_selector,
            position: 0,
            content,
            is_buffering: false,
        }
    }
}

impl body_action::BodyAction for BodyPrepend {
    fn enter(&mut self, data: String) -> (Option<String>, Option<String>, bool, String) {
        let next_leave = Some(self.element_tree[self.position].clone());
        let mut next_enter = None;
        let mut new_data = data.clone();

        if self.position + 1 < self.element_tree.len() {
            self.position += 1;
            next_enter = Some(self.element_tree[self.position].clone());

            return (next_enter, next_leave, false, new_data);
        }

        if self.position + 1 >= self.element_tree.len() {
            if self.css_selector.is_none() {
                new_data.push_str(self.content.as_str());
            } else {
                self.is_buffering = true;
            }
        }

        (next_enter, next_leave, self.is_buffering, new_data)
    }

    fn leave(&mut self, data: String) -> (Option<String>, Option<String>, String) {
        let next_enter = Some(self.element_tree[self.position].clone());
        let next_leave = if self.position as i32 > 0 {
            self.position -= 1;

            Some(self.element_tree[self.position].clone())
        } else {
            None
        };

        if self.is_buffering
            && self.css_selector.is_some()
            && !self.css_selector.as_ref().unwrap().is_empty()
        {
            self.is_buffering = false;

            if !body_action::evaluate(data.clone(), self.css_selector.as_ref().unwrap().clone()) {
                return (
                    next_enter,
                    next_leave,
                    prepend_child(data, self.content.clone()),
                );
            }
        }

        (next_enter, next_leave, data)
    }

    fn first(&self) -> String {
        self.element_tree[0].clone()
    }
}

fn prepend_child(content: String, child: String) -> String {
    let buffer = &mut content.as_bytes() as &mut dyn std::io::Read;
    let mut tokenizer = html::Tokenizer::new(buffer);
    let mut output = "".to_string();

    loop {
        let token_type = tokenizer.next();

        if token_type == html::TokenType::ErrorToken {
            return content;
        }

        if token_type == html::TokenType::StartTagToken {
            output.push_str(tokenizer.raw().as_str());
            output.push_str(child.as_str());
            output.push_str(tokenizer.buffered().as_str());

            return output;
        }

        output.push_str(tokenizer.raw().as_str());
    }
}
