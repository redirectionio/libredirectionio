use crate::filter::html_body_action::HtmlBodyVisitor;
use crate::html;
use std::collections::HashSet;

#[derive(Debug)]
struct BufferLink {
    buffer: String,
    tag_name: String,
    previous: Option<Box<BufferLink>>,
}

#[derive(Debug)]
pub struct HtmlFilterBodyAction {
    enter: Option<String>,
    leave: Option<String>,
    visitor: HtmlBodyVisitor,
    current_buffer: Option<Box<BufferLink>>,
    last_buffer: String,
}

lazy_static! {
    pub static ref VOID_ELEMENTS: HashSet<&'static str> = {
        let mut set = HashSet::new();

        set.insert("area");
        set.insert("base");
        set.insert("br");
        set.insert("col");
        set.insert("embed");
        set.insert("hr");
        set.insert("img");
        set.insert("input");
        set.insert("meta");
        set.insert("param");
        set.insert("source");
        set.insert("track");
        set.insert("wbr");

        set
    };
}

impl HtmlFilterBodyAction {
    pub fn new(visitor: HtmlBodyVisitor) -> Self {
        Self {
            enter: Some(visitor.first()),
            leave: None,
            last_buffer: "".to_string(),
            current_buffer: None,
            visitor,
        }
    }

    pub fn filter(&mut self, input: String) -> String {
        let mut data = self.last_buffer.clone();
        data.push_str(input.as_str());
        let buffer = &mut data.as_bytes() as &mut dyn std::io::Read;
        let mut tokenizer = html::Tokenizer::new(buffer);
        let mut to_return = "".to_string();

        loop {
            let mut token_type = tokenizer.next();

            if token_type == html::TokenType::ErrorToken {
                self.last_buffer = tokenizer.raw();
                self.last_buffer.push_str(tokenizer.buffered().as_str());

                break;
            }

            let mut token_data = tokenizer.raw().clone();

            while token_type == html::TokenType::TextToken && (token_data.contains('<') || token_data.contains("</")) {
                token_type = tokenizer.next();

                if token_type == html::TokenType::ErrorToken {
                    self.last_buffer = token_data;
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

            match token_type {
                html::TokenType::StartTagToken => {
                    let (tag_name, _) = tokenizer.tag_name();
                    let tag_name_str = tag_name.unwrap_or_else(|| "".to_string());
                    let (new_buffer_link, new_token_data) = self.on_start_tag_token(tag_name_str.clone(), token_data);

                    self.current_buffer = new_buffer_link;
                    token_data = new_token_data;

                    if VOID_ELEMENTS.contains(tag_name_str.as_str()) {
                        let (new_buffer_link, new_token_data) = self.on_end_tag_token(tag_name_str.clone(), token_data);

                        self.current_buffer = new_buffer_link;
                        token_data = new_token_data;
                    }
                }
                html::TokenType::EndTagToken => {
                    let (tag_name, _) = tokenizer.tag_name();
                    let (new_buffer_link, new_token_data) = self.on_end_tag_token(tag_name.unwrap(), token_data);

                    self.current_buffer = new_buffer_link;
                    token_data = new_token_data;
                }
                html::TokenType::SelfClosingTagToken => {
                    let (tag_name, _) = tokenizer.tag_name();
                    let (new_buffer_link, new_token_data) = self.on_start_tag_token(tag_name.as_ref().unwrap().clone(), token_data);

                    self.current_buffer = new_buffer_link;
                    token_data = new_token_data;

                    let (new_buffer_link, new_token_data) = self.on_end_tag_token(tag_name.unwrap(), token_data);

                    self.current_buffer = new_buffer_link;
                    token_data = new_token_data;
                }
                _ => {}
            }

            if self.current_buffer.is_some() {
                self.current_buffer.as_mut().unwrap().buffer.push_str(token_data.as_str());
            } else {
                to_return.push_str(token_data.as_str());
            }
        }

        to_return
    }

    pub fn end(&mut self) -> String {
        let mut to_return = self.last_buffer.clone();
        let mut buffer = self.current_buffer.as_ref();

        while buffer.is_some() {
            to_return.push_str(buffer.unwrap().buffer.as_str());
            buffer = buffer.unwrap().previous.as_ref();
        }

        to_return
    }

    fn on_start_tag_token(&mut self, tag_name: String, data: String) -> (Option<Box<BufferLink>>, String) {
        let mut buffer = data;
        let mut buffer_link_actions = 0;

        if self.enter.is_some() && self.enter.as_ref().unwrap() == tag_name.as_str() {
            let (next_enter, next_leave, start_buffer, new_buffer) = self.visitor.enter(buffer);

            buffer = new_buffer;

            self.enter = next_enter;
            self.leave = next_leave;

            if start_buffer {
                buffer_link_actions += 1;
            }
        }

        if buffer_link_actions > 0 {
            let new_current_buffer = BufferLink {
                tag_name,
                previous: self.current_buffer.take(),
                buffer: "".to_string(),
            };

            self.current_buffer = Some(Box::new(new_current_buffer));
        }

        (self.current_buffer.take(), buffer)
    }

    fn on_end_tag_token(&mut self, tag_name: String, data: String) -> (Option<Box<BufferLink>>, String) {
        let mut buffer: String;

        if self.current_buffer.is_some() && self.current_buffer.as_ref().unwrap().tag_name == tag_name {
            buffer = self.current_buffer.as_ref().unwrap().buffer.clone();
            buffer.push_str(data.as_str());
        } else {
            buffer = data;
        }

        if self.leave.is_some() && self.leave.as_ref().unwrap() == tag_name.as_str() {
            let (next_enter, next_leave, new_buffer) = self.visitor.leave(buffer);
            buffer = new_buffer;

            self.enter = next_enter;
            self.leave = next_leave;
        }

        if self.current_buffer.is_some() && self.current_buffer.as_ref().unwrap().tag_name == tag_name {
            return (self.current_buffer.as_mut().unwrap().previous.take(), buffer);
        }

        (self.current_buffer.take(), buffer)
    }
}
