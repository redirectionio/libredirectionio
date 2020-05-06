use crate::filter::header_action::HeaderAction;
use crate::http::Header;

#[derive(Debug)]
pub struct HeaderAddAction {
    pub name: String,
    pub value: String,
}

impl HeaderAction for HeaderAddAction {
    fn filter(&self, mut headers: Vec<Header>) -> Vec<Header> {
        headers.push(Header {
            name: self.name.clone(),
            value: self.value.clone(),
        });

        headers
    }
}
