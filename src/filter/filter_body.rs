use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::string;
use crate::filter::body_action;
use xmlparser::{ElementEnd, Token, Tokenizer};
use crate::html;


struct BufferLink {
    actions: Vec<Box<body_action::BodyAction>>,
    buffer: String,
    tag_name: String,
}

struct FilterBodyAction {
    //    enter_visitors: HashMap<String, Vec<Box<BodyAction>>>,
//    leave_visitors: HashMap<String, Vec<Box<BodyAction>>>,
    current_buffer: Option<BufferLink>,
    last_buffer: String,
}

impl FilterBodyAction {
    pub fn filter(&mut self, mut input: String) -> String {
        let mut data = self.last_buffer.clone();
        data.push_str(input.as_str());
        let mut buffer = &mut data.as_bytes() as &mut std::io::Read;
        let mut tokenizer = html::Tokenizer::new(buffer);
        let mut to_return = "".to_string();

        loop {
            let mut token_type = tokenizer.next();

            if token_type == html::TokenType::ErrorToken {
                self.last_buffer = tokenizer.raw().clone();
                self.last_buffer.push_str(tokenizer.buffered().as_str());

                break;
            }

            let mut token_data = tokenizer.raw().clone();

            while token_type == html::TokenType::TextToken && (token_data.contains("<") || token_data.contains("</")) {
                token_type = tokenizer.next();

                if token_type == html::TokenType::ErrorToken {
                    self.last_buffer = token_data.clone();
                    self.last_buffer.push_str(tokenizer.raw().as_str());
                    self.last_buffer.push_str(tokenizer.buffered().as_str());

                    return to_return;
                }

                if self.current_buffer.is_some() {
                    self.current_buffer.as_mut().unwrap().buffer.push_str(token_data.as_str());
                } else {
                    to_return.push_str(token_data.as_str());
                }

                token_data = tokenizer.raw();
            }

            if token_type == html::TokenType::StartTagToken {
                let (tag_name, _) = tokenizer.tag_name();
                println!("On start tag {}", tag_name.unwrap());
            }

            if token_type == html::TokenType::EndTagToken {
                let (tag_name, _) = tokenizer.tag_name();
                println!("On end tag {}", tag_name.unwrap());
            }

            if token_type == html::TokenType::SelfClosingTagToken {
                let (tag_name, _) = tokenizer.tag_name();
                println!("On self closing tag tag {}", tag_name.unwrap());
            }

            if self.current_buffer.is_some() {
                self.current_buffer.as_mut().unwrap().buffer.push_str(token_data.as_str());
            } else {
                to_return.push_str(token_data.as_str());
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
        let mut filter = FilterBodyAction {
            last_buffer: "".to_string(),
            current_buffer: None,
        };

        filter.filter(
            "Text Text Text </test><html><head><meta attribute=\"yolo\" /></head><body>Text />baddattr=\"tata\" Text <a/><a/></body></html>"
                .to_string(),
        );
    }

    #[test]
    pub fn test_error() {
        let mut filter = FilterBodyAction {
            last_buffer: "".to_string(),
            current_buffer: None,
        };

        filter.filter(
            "<div>Text </ Text</div>"
                .to_string(),
        );
    }
}
