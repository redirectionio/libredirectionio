use crate::filter::body_action;
use crate::html;
use crate::router::rule;
use std::collections::HashMap;

#[derive(Debug)]
struct BufferLink {
    buffer: String,
    tag_name: String,
    previous: Option<Box<BufferLink>>,
}

#[derive(Debug)]
struct FilterBodyVisitor {
    enter: Option<String>,
    leave: Option<String>,
    action: Box<body_action::BodyAction>,
}

#[derive(Debug)]
pub struct FilterBodyAction {
    visitors: Vec<FilterBodyVisitor>,
    current_buffer: Option<Box<BufferLink>>,
    last_buffer: String,
}

lazy_static! {
    static ref VOID_ELEMENTS: HashMap<&'static str, bool> = {
        let mut map = HashMap::new();
        map.insert("area", true);
        map.insert("base", true);
        map.insert("br", true);
        map.insert("col", true);
        map.insert("embed", true);
        map.insert("hr", true);
        map.insert("img", true);
        map.insert("input", true);
        map.insert("meta", true);
        map.insert("param", true);
        map.insert("source", true);
        map.insert("track", true);
        map.insert("wbr", true);

        map
    };
}

impl FilterBodyAction {
    pub fn new(rule_to_filter: rule::Rule) -> Option<FilterBodyAction> {
        if rule_to_filter.body_filters.is_none() {
            return None;
        }

        let mut visitors = Vec::new();

        for filter in rule_to_filter.body_filters.as_ref().unwrap() {
            let action = body_action::create_body_action(filter);

            if action.is_some() {
                let action_unwrap = action.unwrap();
                let visitor = FilterBodyVisitor {
                    enter: Some(action_unwrap.first()),
                    leave: None,
                    action: action_unwrap,
                };

                visitors.push(visitor);
            }
        }

        if visitors.len() > 0 {
            return Some(FilterBodyAction {
                visitors,
                last_buffer: "".to_string(),
                current_buffer: None,
            });
        }

        return None;
    }

    pub fn filter(&mut self, input: String) -> String {
        let mut data = self.last_buffer.clone();
        data.push_str(input.as_str());
        let buffer = &mut data.as_bytes() as &mut std::io::Read;
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

            while token_type == html::TokenType::TextToken
                && (token_data.contains("<") || token_data.contains("</"))
            {
                token_type = tokenizer.next();

                if token_type == html::TokenType::ErrorToken {
                    self.last_buffer = token_data.clone();
                    self.last_buffer.push_str(tokenizer.raw().as_str());
                    self.last_buffer.push_str(tokenizer.buffered().as_str());

                    return to_return;
                }

                if self.current_buffer.is_some() {
                    self.current_buffer
                        .as_mut()
                        .unwrap()
                        .buffer
                        .push_str(token_data.as_str());
                } else {
                    to_return.push_str(token_data.as_str());
                }

                token_data = tokenizer.raw();
            }

            match token_type {
                html::TokenType::StartTagToken => {
                    let (tag_name, _) = tokenizer.tag_name();
                    let tag_name_str = tag_name.unwrap_or("".to_string());
                    let (new_buffer_link, new_token_data) =
                        self.on_start_tag_token(tag_name_str.clone(), token_data);

                    self.current_buffer = new_buffer_link;
                    token_data = new_token_data;

                    if VOID_ELEMENTS.get(tag_name_str.as_str()).is_some() {
                        let (new_buffer_link, new_token_data) = self.on_end_tag_token(tag_name_str.clone(), token_data);

                        self.current_buffer = new_buffer_link;
                        token_data = new_token_data;
                    }
                }
                html::TokenType::EndTagToken => {
                    let (tag_name, _) = tokenizer.tag_name();
                    let (new_buffer_link, new_token_data) =
                        self.on_end_tag_token(tag_name.unwrap(), token_data);

                    self.current_buffer = new_buffer_link;
                    token_data = new_token_data;
                }
                html::TokenType::SelfClosingTagToken => {
                    let (tag_name, _) = tokenizer.tag_name();
                    let (new_buffer_link, new_token_data) =
                        self.on_start_tag_token(tag_name.as_ref().unwrap().clone(), token_data);

                    self.current_buffer = new_buffer_link;
                    token_data = new_token_data;

                    let (new_buffer_link, new_token_data) =
                        self.on_end_tag_token(tag_name.unwrap(), token_data);

                    self.current_buffer = new_buffer_link;
                    token_data = new_token_data;
                }
                _ => {}
            }

            if self.current_buffer.is_some() {
                self.current_buffer
                    .as_mut()
                    .unwrap()
                    .buffer
                    .push_str(token_data.as_str());
            } else {
                to_return.push_str(token_data.as_str());
            }
        }

        return to_return;
    }

    pub fn end(&mut self) -> String {
        let mut to_return = self.last_buffer.clone();
        let mut buffer = self.current_buffer.as_ref();

        while buffer.is_some() {
            to_return.push_str(buffer.unwrap().buffer.as_str());
            buffer = buffer.unwrap().previous.as_ref();
        }

        return to_return;
    }

    fn on_start_tag_token(
        &mut self,
        tag_name: String,
        data: String,
    ) -> (Option<Box<BufferLink>>, String) {
        let mut buffer = data.clone();
        let mut buffer_link_actions = 0;

        for visitor in &mut self.visitors {
            if visitor.enter.is_some() && visitor.enter.as_ref().unwrap() == tag_name.as_str() {
                let (next_enter, next_leave, start_buffer, new_buffer) =
                    visitor.action.enter(buffer);

                buffer = new_buffer;

                visitor.enter = next_enter;
                visitor.leave = next_leave;

                if start_buffer {
                    buffer_link_actions += 1;
                }
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

        return (self.current_buffer.take(), buffer);
    }

    fn on_end_tag_token(
        &mut self,
        tag_name: String,
        data: String,
    ) -> (Option<Box<BufferLink>>, String) {
        let mut buffer: String;

        if self.current_buffer.is_some() && self.current_buffer.as_ref().unwrap().tag_name == tag_name {
            buffer = self.current_buffer.as_ref().unwrap().buffer.clone();
            buffer.push_str(data.as_str());
        } else {
            buffer = data.clone();
        }

        for visitor in &mut self.visitors {
            if visitor.leave.is_some() && visitor.leave.as_ref().unwrap() == tag_name.as_str() {
                let (next_enter, next_leave, new_buffer) = visitor.action.leave(buffer);
                buffer = new_buffer;

                visitor.enter = next_enter;
                visitor.leave = next_leave;
            }
        }

        if self.current_buffer.is_some()
            && self.current_buffer.as_ref().unwrap().tag_name == tag_name
        {
            return (
                self.current_buffer.as_mut().unwrap().previous.take(),
                buffer,
            );
        }

        return (self.current_buffer.take(), buffer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::body_action::body_replace::BodyReplace;
    use crate::filter::body_action::body_append::BodyAppend;

    #[test]
    pub fn test_filter() {
        let mut filter = FilterBodyAction {
            last_buffer: "".to_string(),
            current_buffer: None,
            visitors: Vec::new(),
        };

        let before_filter = "Test".to_string();
        let filtered = filter.filter(before_filter.clone());
        let end = filter.end();

        assert_eq!(before_filter, filtered);
        assert_eq!(true, end.is_empty());
    }

    #[test]
    pub fn test_buffer_on_error() {
        let mut filter = FilterBodyAction {
            last_buffer: "".to_string(),
            current_buffer: None,
            visitors: Vec::new(),
        };

        let filtered = filter.filter("<div>Text </".to_string());
        let end = filter.end();

        assert_eq!("<div>Text ", filtered);
        assert_eq!("</", end);
    }

    #[test]
    pub fn test_append_and_replace() {
        let mut visitors = Vec::new();
        let append  = Box::new(BodyAppend::new(vec!["html".to_string(), "head".to_string()], Some("count(//meta[@name = 'description']) = 0".to_string()), "<meta name=\"description\" content=\"New Description\" />".to_string()));
        let replace = Box::new(BodyReplace::new(vec!["html".to_string(), "head".to_string(), "meta".to_string()], Some("count(//meta[@name = 'description']) = 1".to_string()), "<meta name=\"description\" content=\"New Description\" />".to_string()));

        visitors.push(FilterBodyVisitor {
            enter: Some("html".to_string()),
            leave: None,
            action: append,
        });

        visitors.push(FilterBodyVisitor {
            enter: Some("html".to_string()),
            leave: None,
            action: replace,
        });

        let mut filter = FilterBodyAction {
            last_buffer: "".to_string(),
            current_buffer: None,
            visitors,
        };

        let mut filtered = filter.filter("<html><head><meta name=\"description\"></head></html>".to_string());
        let end = filter.end();

        filtered.push_str(end.as_str());

        assert_eq!("<html><head><meta name=\"description\" content=\"New Description\" /></head></html>", filtered);
    }
}
