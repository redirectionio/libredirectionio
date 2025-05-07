use super::{super::html_filter_body::VOID_ELEMENTS, evaluate};
use crate::{action::UnitTrace, filter::error::Result, html};

#[derive(Debug)]
pub struct BodyAppend {
    element_tree: Vec<String>,
    position: usize,
    css_selector: Option<String>,
    content: String,
    inner_content: String,
    id: Option<String>,
    target_hash: Option<String>,
}

impl BodyAppend {
    pub fn new(
        element_tree: Vec<String>,
        css_selector: Option<String>,
        content: String,
        inner_content: String,
        id: Option<String>,
        target_hash: Option<String>,
    ) -> BodyAppend {
        BodyAppend {
            element_tree,
            css_selector,
            position: 0,
            content,
            inner_content,
            id,
            target_hash,
        }
    }
}

impl BodyAppend {
    pub fn enter(&mut self, data: String) -> (Option<String>, Option<String>, bool, String) {
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

    pub fn leave(&mut self, data: String, unit_trace: Option<&mut UnitTrace>) -> Result<(Option<String>, Option<String>, String)> {
        let next_enter = Some(self.element_tree[self.position].clone());
        let is_processing = self.position + 1 >= self.element_tree.len();
        let next_leave = if self.position as i32 > 0 {
            self.position -= 1;

            Some(self.element_tree[self.position].clone())
        } else {
            None
        };

        if is_processing {
            if let Some(css_selector) = self.css_selector.as_ref() {
                if !css_selector.is_empty() {
                    if !evaluate(data.as_str(), css_selector.as_str()) {
                        if let Some(trace) = unit_trace {
                            if let Some(id) = self.id.clone() {
                                trace.add_value_computed_by_unit(&id, &self.inner_content);
                                if let Some(target_hash) = self.target_hash.clone() {
                                    trace.add_unit_id_with_target(target_hash.as_str(), id.as_str());
                                } else {
                                    trace.add_unit_id(id);
                                }
                            }
                        }

                        return Ok((next_enter, next_leave, append_child(data, self.content.clone())?));
                    }

                    return Ok((next_enter, next_leave, data));
                }
            }

            let mut new_data = self.content.clone();
            if let Some(trace) = unit_trace {
                if let Some(id) = self.id.clone() {
                    trace.add_value_computed_by_unit(&id, &self.inner_content);
                    if let Some(target_hash) = self.target_hash.clone() {
                        trace.add_unit_id_with_target(target_hash.as_str(), id.as_str());
                    } else {
                        trace.add_unit_id(id);
                    }
                }
            }
            new_data.push_str(data.as_str());

            return Ok((next_enter, next_leave, new_data));
        }

        Ok((next_enter, next_leave, data))
    }

    pub fn first(&self) -> String {
        self.element_tree[0].clone()
    }
}

fn append_child(content: String, child: String) -> Result<String> {
    let buffer = content.as_bytes().to_vec();
    let mut tokenizer = html::Tokenizer::new(buffer);
    let mut output = "".to_string();
    let mut level = 0;

    loop {
        let token_type = tokenizer.next()?;

        if token_type == html::TokenType::ErrorToken {
            return Ok(content);
        }

        if token_type == html::TokenType::StartTagToken {
            level += 1;
            let (tag_name, _) = tokenizer.tag_name()?;

            if VOID_ELEMENTS.contains(tag_name.unwrap().as_str()) {
                level -= 1;
            }
        }

        if token_type == html::TokenType::EndTagToken {
            level -= 1;

            if level == 0 {
                output.push_str(child.as_str());
                output.push_str(tokenizer.raw_as_string()?.as_str());
                output.push_str(tokenizer.buffered_as_string()?.as_str());

                return Ok(output);
            }
        }

        output.push_str(tokenizer.raw_as_string()?.as_str());
    }
}
