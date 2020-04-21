use crate::filter::header_action::HeaderAction;
use crate::api::MessageHeader;

#[derive(Debug)]
pub struct HeaderRemoveAction {
    pub name: String,
}

impl HeaderAction for HeaderRemoveAction {
    fn filter(&self, headers: Vec<MessageHeader>) -> Vec<MessageHeader> {
        let mut new_headers = Vec::new();

        for header in headers {
            if header.name != self.name {
                new_headers.push(header);
            }
        }

        new_headers
    }
}
