use std::{cell::RefCell, rc::Rc};

use html_to_markdown_rs::{ConversionOptions, convert};

use crate::action::UnitTrace;

#[derive(Debug)]
pub struct HtmlToMarkdownFilter {
    buffer: Vec<u8>,
    options: Option<ConversionOptions>,
    id: Option<String>,
    unit_trace: Option<Rc<RefCell<UnitTrace>>>,
    executed: bool,
}

impl HtmlToMarkdownFilter {
    pub fn new(options: Option<ConversionOptions>, id: Option<String>, unit_trace: Option<Rc<RefCell<UnitTrace>>>) -> Self {
        Self {
            buffer: Vec::new(),
            options,
            id,
            unit_trace,
            executed: false,
        }
    }

    pub fn filter(&mut self, input: Vec<u8>) -> Vec<u8> {
        self.buffer.extend_from_slice(&input);
        Vec::new()
    }

    pub fn end(mut self) -> Vec<u8> {
        if !self.executed {
            self.executed = true;

            if let Some(trace) = self.unit_trace
                && let Some(id) = self.id
            {
                trace.borrow_mut().override_unit_id_with_target("text", id.as_str());
            }
        }

        let html = match String::from_utf8(self.buffer) {
            Err(e) => {
                tracing::error!("error while converting to utf8: {}", e);

                return e.into_bytes();
            }
            Ok(html) => html,
        };

        match convert(html.as_str(), self.options) {
            Ok(md) => md.content.unwrap_or_default().into_bytes(),
            Err(e) => {
                tracing::error!("error while converting html to markdown: {}", e);

                html.into_bytes()
            }
        }
    }
}
