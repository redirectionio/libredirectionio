#[derive(Debug)]
pub struct TextFilterBodyAction {
    action: TextFilterAction,
    content: String,
    executed: bool,
}

#[derive(Debug)]
pub enum TextFilterAction {
    Append,
    Prepend,
    Replace,
}

impl TextFilterBodyAction {
    pub fn new(action: TextFilterAction, content: String) -> Self {
        Self {
            action,
            content,
            executed: false,
        }
    }

    pub fn filter(&mut self, data: String) -> String {
        match self.action {
            TextFilterAction::Replace => {
                if self.executed {
                    "".to_string()
                } else {
                    self.executed = true;
                    self.content.clone()
                }
            }
            TextFilterAction::Append => data,
            TextFilterAction::Prepend => {
                if self.executed {
                    data
                } else {
                    self.executed = true;
                    let mut content = self.content.clone();
                    content.push_str(data.as_str());

                    content
                }
            }
        }
    }

    pub fn end(&mut self) -> String {
        if self.executed {
            "".to_string()
        } else {
            self.executed = true;
            self.content.clone()
        }
    }
}
