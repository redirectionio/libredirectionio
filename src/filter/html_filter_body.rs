use std::{cell::RefCell, rc::Rc};

use lol_html::{HtmlRewriter, OutputSink, Settings};

use crate::filter::{error::Result, html_body_action::HtmlBodyVisitor};

#[derive(Debug)]
pub struct HtmlFilterBodyAction {
    output: Rc<RefCell<Vec<u8>>>,
    rewriter: HtmlRewriter<'static, OutputSinkBuffer>,
}

pub struct OutputSinkBuffer {
    output: Rc<RefCell<Vec<u8>>>,
}

impl OutputSink for OutputSinkBuffer {
    fn handle_chunk(&mut self, chunk: &[u8]) {
        self.output.borrow_mut().extend_from_slice(chunk);
    }
}

impl HtmlFilterBodyAction {
    pub fn new(visitor: HtmlBodyVisitor) -> Self {
        let output = Rc::new(RefCell::new(vec![]));
        let mut settings = Settings::default();
        visitor.into_handlers(&mut settings);

        let rewriter = HtmlRewriter::new(settings, OutputSinkBuffer { output: output.clone() });

        Self { output, rewriter }
    }

    pub fn filter(&mut self, input: Vec<u8>) -> Result<Vec<u8>> {
        match self.rewriter.write(&input) {
            Ok(_) => {}
            Err(err) => {
                tracing::error!("Error while parsing html: {}", err);
            }
        }

        let output = RefCell::new(vec![]);
        self.output.swap(&output);

        Ok(output.take())
    }

    pub fn end(self) -> Vec<u8> {
        match self.rewriter.end() {
            Ok(_) => {}
            Err(err) => {
                tracing::error!("Error while ending html rewriter: {}", err);
            }
        }

        self.output.take()
    }
}
