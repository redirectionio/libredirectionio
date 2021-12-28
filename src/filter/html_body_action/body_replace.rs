use crate::action::UnitTrace;

use super::evaluate;

#[derive(Debug)]
pub struct BodyReplace {
    element_tree: Vec<String>,
    position: usize,
    css_selector: Option<String>,
    content: String,
    inner_content: String,
    is_buffering: bool,
    id: Option<String>,
    target_hash: Option<String>,
}

impl BodyReplace {
    pub fn new(
        element_tree: Vec<String>,
        css_selector: Option<String>,
        content: String,
        inner_content: String,
        id: Option<String>,
        target_hash: Option<String>,
    ) -> BodyReplace {
        BodyReplace {
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

impl BodyReplace {
    pub fn enter(&mut self, data: String) -> (Option<String>, Option<String>, bool, String) {
        let next_leave = Some(self.element_tree[self.position].clone());
        let mut next_enter = None;

        if self.position + 1 < self.element_tree.len() {
            self.position += 1;
            next_enter = Some(self.element_tree[self.position].clone());

            return (next_enter, next_leave, false, data);
        }

        if self.position + 1 >= self.element_tree.len() {
            self.is_buffering = true;

            return (next_enter, next_leave, true, data);
        }

        (next_enter, next_leave, false, data)
    }

    pub fn leave(&mut self, data: String, mut unit_trace: Option<&mut UnitTrace>) -> (Option<String>, Option<String>, String) {
        let next_enter = Some(self.element_tree[self.position].clone());

        let next_leave = if self.position as i32 > 0 && !self.is_buffering {
            self.position -= 1;
            Some(self.element_tree[self.position].clone())
        } else {
            None
        };

        if self.is_buffering {
            self.is_buffering = false;

            if self.css_selector.is_none() || self.css_selector.as_ref().unwrap().is_empty() {
                if let Some(trace) = unit_trace.as_deref_mut() {
                    if let Some(id) = self.id.clone() {
                        trace.add_value_computed_by_unit(&id, &self.inner_content);
                        if let Some(target_hash) = self.target_hash.clone() {
                            trace.override_unit_id_with_target(target_hash.as_str(), id.as_str());
                        } else {
                            trace.add_unit_id(id);
                        }
                    }
                }
                return (next_enter, next_leave, self.content.clone());
            }

            if evaluate(data.as_str(), self.css_selector.as_ref().unwrap().as_str()) {
                if let Some(trace) = unit_trace.as_deref_mut() {
                    if let Some(id) = self.id.clone() {
                        trace.add_value_computed_by_unit(&id, &self.inner_content);
                        if let Some(target_hash) = self.target_hash.clone() {
                            trace.override_unit_id_with_target(target_hash.as_str(), id.as_str());
                        } else {
                            trace.add_unit_id(id);
                        }
                    }
                }
                return (next_enter, next_leave, self.content.clone());
            }
        }

        (next_enter, next_leave, data)
    }

    pub fn first(&self) -> String {
        self.element_tree[0].clone()
    }
}
