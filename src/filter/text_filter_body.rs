use crate::action::UnitTrace;

#[derive(Debug)]
pub struct TextFilterBodyAction {
    id: Option<String>,
    action: TextFilterAction,
    content: Vec<u8>,
    executed: bool,
}

#[derive(Debug)]
pub enum TextFilterAction {
    Append,
    Prepend,
    Replace,
}

impl TextFilterBodyAction {
    pub fn new(id: Option<String>, action: TextFilterAction, content: String) -> Self {
        Self {
            id,
            action,
            content: content.into_bytes(),
            executed: false,
        }
    }

    pub fn filter(&mut self, data: Vec<u8>, unit_trace: Option<&mut UnitTrace>) -> Vec<u8> {
        match self.action {
            TextFilterAction::Replace => {
                if let Some(trace) = unit_trace {
                    if let Some(id) = self.id.clone() {
                        // We always use "body" as target since it's not
                        // possible to change the value in the UI
                        trace.override_unit_id_with_target("text", id.as_str());
                    }
                }

                if self.executed {
                    Vec::new()
                } else {
                    self.executed = true;
                    self.content.clone()
                }
            }
            TextFilterAction::Append => {
                if let Some(trace) = unit_trace {
                    if let Some(id) = self.id.clone() {
                        // We always use "body" as target since it's not
                        // possible to change the value in the UI
                        trace.add_unit_id_with_target("text", id.as_str());
                    }
                }

                data
            }
            TextFilterAction::Prepend => {
                if let Some(trace) = unit_trace {
                    if let Some(id) = self.id.clone() {
                        // We always use "body" as target since it's not
                        // possible to change the value in the UI
                        trace.add_unit_id_with_target("text", id.as_str());
                    }
                }

                if self.executed {
                    data
                } else {
                    self.executed = true;
                    let mut content = self.content.clone();
                    content.extend(data);

                    content
                }
            }
        }
    }

    pub fn end(&mut self) -> Vec<u8> {
        if self.executed {
            Vec::new()
        } else {
            self.executed = true;
            self.content.clone()
        }
    }
}
