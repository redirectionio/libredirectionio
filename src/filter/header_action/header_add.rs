use crate::filter::header_action::HeaderAction;
use crate::api::MessageHeader;

#[derive(Debug)]
pub struct HeaderAddAction {
    pub name: String,
    pub value: String,
}

impl HeaderAction for HeaderAddAction {
    fn filter(&self, mut headers: Vec<MessageHeader>) -> Vec<MessageHeader> {
        headers.push(MessageHeader{
            name: self.name.clone(),
            value: self.value.clone(),
        });

        headers
    }
}
