use super::evaluate;
use crate::filter::error::Result;
use crate::{action::UnitTrace, html};

#[derive(Debug)]
pub struct BodyPrepend {
    element_tree: Vec<String>,
    position: usize,
    css_selector: Option<String>,
    content: String,
    inner_content: String,
    is_buffering: bool,
    id: Option<String>,
    target_hash: Option<String>,
}

impl BodyPrepend {
    pub fn new(
        element_tree: Vec<String>,
        css_selector: Option<String>,
        content: String,
        inner_content: String,
        id: Option<String>,
        target_hash: Option<String>,
    ) -> BodyPrepend {
        BodyPrepend {
            element_tree,
            css_selector,
            position: 0,
            content,
            inner_content,
            is_buffering: false,
            id,
            target_hash,
        }
    }
}

impl BodyPrepend {
    pub fn enter(&mut self, data: String, mut unit_trace: Option<&mut UnitTrace>) -> (Option<String>, Option<String>, bool, String) {
        let next_leave = Some(self.element_tree[self.position].clone());
        let mut next_enter = None;
        let mut new_data = data;

        if self.position + 1 < self.element_tree.len() {
            self.position += 1;
            next_enter = Some(self.element_tree[self.position].clone());

            return (next_enter, next_leave, false, new_data);
        }

        if self.position + 1 >= self.element_tree.len() {
            if self.css_selector.is_none() || self.css_selector.as_ref().unwrap().is_empty() {
                new_data.push_str(self.content.as_str());
                if let Some(trace) = unit_trace.as_deref_mut() {
                    if let Some(id) = self.id.clone() {
                        trace.add_value_computed_by_unit(&id, &self.inner_content);
                        if let Some(target_hash) = self.target_hash.clone() {
                            trace.add_unit_id_with_target(target_hash.as_str(), id.as_str());
                        } else {
                            trace.add_unit_id(id);
                        }
                    }
                }
            } else {
                self.is_buffering = true;
            }
        }

        (next_enter, next_leave, self.is_buffering, new_data)
    }

    pub fn leave(&mut self, data: String, mut unit_trace: Option<&mut UnitTrace>) -> Result<(Option<String>, Option<String>, String)> {
        let next_enter = Some(self.element_tree[self.position].clone());
        let next_leave = if self.position as i32 > 0 {
            self.position -= 1;

            Some(self.element_tree[self.position].clone())
        } else {
            None
        };

        if self.is_buffering && self.css_selector.is_some() && !self.css_selector.as_ref().unwrap().is_empty() {
            self.is_buffering = false;

            if !evaluate(data.as_str(), self.css_selector.as_ref().unwrap().as_str()) {
                if let Some(trace) = unit_trace.as_deref_mut() {
                    if let Some(id) = self.id.clone() {
                        trace.add_value_computed_by_unit(&id, &self.inner_content);
                        if let Some(target_hash) = self.target_hash.clone() {
                            trace.add_unit_id_with_target(target_hash.as_str(), id.as_str());
                        } else {
                            trace.add_unit_id(id);
                        }
                    }
                }
                return Ok((next_enter, next_leave, prepend_child(data, self.content.clone())?));
            }
        }

        Ok((next_enter, next_leave, data))
    }

    pub fn first(&self) -> String {
        self.element_tree[0].clone()
    }
}

fn prepend_child(content: String, child: String) -> Result<String> {
    let buffer = &mut content.as_bytes() as &mut dyn std::io::Read;
    let mut tokenizer = html::Tokenizer::new(buffer);
    let mut output = "".to_string();

    loop {
        let token_type = tokenizer.next()?;

        if token_type == html::TokenType::ErrorToken {
            return Ok(content);
        }

        if token_type == html::TokenType::StartTagToken {
            output.push_str(tokenizer.raw_as_string()?.as_str());
            output.push_str(child.as_str());
            output.push_str(tokenizer.buffered_as_string()?.as_str());

            return Ok(output);
        }

        output.push_str(tokenizer.raw_as_string()?.as_str());
    }
}
