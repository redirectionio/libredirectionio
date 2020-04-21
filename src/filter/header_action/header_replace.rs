use crate::filter::header_action::HeaderAction;
use crate::api::MessageHeader;

#[derive(Debug)]
pub struct HeaderReplaceAction {
    pub name: String,
    pub value: String,
}

impl HeaderAction for HeaderReplaceAction {
    fn filter(&self, headers: Vec<MessageHeader>) -> Vec<MessageHeader> {
        let mut new_headers = Vec::new();

        for header in headers {
            if header.name != self.name {
                new_headers.push(header);
            } else {
                new_headers.push(MessageHeader {
                    name: self.name.clone(),
                    value: self.value.clone(),
                });
            }
        }

        new_headers
    }
}
