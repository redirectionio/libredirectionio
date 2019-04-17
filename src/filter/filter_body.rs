use std::collections::HashMap;
use std::io;
use std::io::Write;
use crate::filter::body_action;
use xmlparser::{ElementEnd, Token, Tokenizer};


struct BufferLink {
    actions: Vec<Box<body_action::BodyAction>>,
    buffer: Vec<u8>,
    tag_name: String,
}

struct FilterBodyAction {
    //    enter_visitors: HashMap<String, Vec<Box<BodyAction>>>,
//    leave_visitors: HashMap<String, Vec<Box<BodyAction>>>,
//    current_buffer: BufferLink,
}

impl FilterBodyAction {
    pub fn filter(&mut self, mut input: String) -> String {
        let mut tokenizer = Tokenizer::from(input.as_str());
        tokenizer.enable_fragment_mode();

        let to_return = "".to_string();

        for token in tokenizer {
            if token.is_err() {
                let error = token.err().unwrap();
//                println!("Invalid token {:?}", error);

                continue;
            }

            let current_token = token.unwrap();

            match current_token {
                Token::ElementStart {
                    prefix,
                    local,
                    span,
                } => {
//                    println!("Element Start {}", span);
                }
                Token::ElementEnd { end, span } => {
//                    println!("Element End {}", span);
                }
                _ => {
//                    println!("{:?}", current_token);
                }
            }
        }

        return to_return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_filter() {
        let mut filter = FilterBodyAction {};

        filter.filter(
            "Text Text Text </test><html><head><meta attribute=\"yolo\" /></head><body>Text />baddattr=\"tata\" Text <a/><a/></body></html>"
                .to_string(),
        );
    }

    #[test]
    pub fn test_error() {
        let mut filter = FilterBodyAction {};

        filter.filter(
            "<div>Text </ Text</div>"
                .to_string(),
        );
    }
}
