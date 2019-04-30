use crate::filter::body_action;
use crate::html;

#[derive(Debug)]
pub struct BodyAppend {
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
        let next_enter = Some(self.element_tree[self.position].clone());
        let mut next_leave = None;
        let is_processing = self.position + 1 >= self.element_tree.len();

        if self.position as i32 - 1 >= 0 {
            self.position = self.position - 1;

            next_leave = Some(self.element_tree[self.position].clone());
        }

        if is_processing {
            if self.x_path_matcher.is_some() {
                if body_action::evaluate(
                    data.clone(),
                    self.x_path_matcher.as_ref().unwrap().clone(),
                ) {
                    return (
                        next_enter,
                        next_leave,
                        append_child(data, self.content.clone()),
                    );
                }

                return (next_enter, next_leave, data);
            }

            let mut new_data = self.content.clone();
            new_data.push_str(data.as_str());

            return (next_enter, next_leave, new_data);
        }

        return (next_enter, next_leave, data);
    }

    fn first(&self) -> String {
        return self.element_tree[0].clone();
    }
}

fn append_child(content: String, child: String) -> String {
    let buffer = &mut content.as_bytes() as &mut std::io::Read;
    let mut tokenizer = html::Tokenizer::new(buffer);
    let mut output = "".to_string();
    let mut level = 0;

    loop {
        let token_type = tokenizer.next();

        if token_type == html::TokenType::ErrorToken {
            return content;
        }

        if token_type == html::TokenType::StartTagToken {
            level += 1;
        }

        if token_type == html::TokenType::EndTagToken {
            level -= 1;

            if level == 0 {
                output.push_str(child.as_str());
                output.push_str(tokenizer.raw().as_str());
                output.push_str(tokenizer.buffered().as_str());

                return output;
            }
        }

        output.push_str(tokenizer.raw().as_str());
    }
}
