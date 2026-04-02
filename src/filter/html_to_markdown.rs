use html_to_markdown_rs::{ConversionOptions, convert};

#[derive(Debug)]
pub struct HtmlToMarkdownFilter {
    buffer: Vec<u8>,
    options: Option<ConversionOptions>,
}

impl HtmlToMarkdownFilter {
    pub fn new(options: Option<ConversionOptions>) -> Self {
        Self {
            buffer: Vec::new(),
            options,
        }
    }

    pub fn filter(&mut self, input: Vec<u8>) -> Vec<u8> {
        self.buffer.extend_from_slice(&input);
        Vec::new()
    }

    pub fn end(self) -> Vec<u8> {
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
