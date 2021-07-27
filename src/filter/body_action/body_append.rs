use crate::filter::body_action;
use crate::filter::html_filter_body;
use crate::html;

#[derive(Debug)]
pub struct BodyAppend {
    element_tree: Vec<String>,
    position: usize,
    css_selector: Option<String>,
    content: String,
    is_buffering: bool,
}

impl BodyAppend {
    pub fn new(element_tree: Vec<String>, css_selector: Option<String>, content: String) -> BodyAppend {
        BodyAppend {
            element_tree,
            css_selector,
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
            self.position += 1;
            next_enter = Some(self.element_tree[self.position].clone());

            return (next_enter, next_leave, false, data);
        }

        let should_buffer =
            self.position + 1 >= self.element_tree.len() && self.css_selector.is_some() && !self.css_selector.as_ref().unwrap().is_empty();

        (next_enter, next_leave, should_buffer, data)
    }

    fn leave(&mut self, data: String) -> (Option<String>, Option<String>, String) {
        let next_enter = Some(self.element_tree[self.position].clone());
        let is_processing = self.position + 1 >= self.element_tree.len();
        let next_leave = if self.position as i32 > 0 {
            self.position -= 1;

            Some(self.element_tree[self.position].clone())
        } else {
            None
        };

        if is_processing {
            if self.css_selector.is_some() && !self.css_selector.as_ref().unwrap().is_empty() {
                if !body_action::evaluate(data.clone(), self.css_selector.as_ref().unwrap().clone()) {
                    return (next_enter, next_leave, append_child(data, self.content.clone()));
                }

                return (next_enter, next_leave, data);
            }

            let mut new_data = self.content.clone();
            new_data.push_str(data.as_str());

            return (next_enter, next_leave, new_data);
        }

        (next_enter, next_leave, data)
    }

    fn first(&self) -> String {
        self.element_tree[0].clone()
    }
}

fn append_child(content: String, child: String) -> String {
    let buffer = &mut content.as_bytes() as &mut dyn std::io::Read;
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
            let (tag_name, _) = tokenizer.tag_name();

            if html_filter_body::VOID_ELEMENTS.contains(tag_name.unwrap().as_str()) {
                level -= 1;
            }
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
