#[derive(Debug)]
pub struct TextFilterBodyAction {
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
    pub fn new(action: TextFilterAction, content: String) -> Self {
        Self {
            action,
            content: content.into_bytes(),
            executed: false,
        }
    }

    pub fn filter(&mut self, data: Vec<u8>) -> Vec<u8> {
        match self.action {
            TextFilterAction::Replace => {
                if self.executed {
                    Vec::new()
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
