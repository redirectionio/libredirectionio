extern crate html5ever;

use html5ever::tokenizer::{Tokenizer, TokenizerOpts};
use std::collections::HashMap;
use std::io;

trait BodyAction {}

struct BufferLink {
    actions: Vec<BodyAction>,
    buffer: Vec<u8>,
    previous: BufferLink,
    tag_name: String,
}

struct FilterBodyAction {
    enter_visitors: HashMap<String, Vec<BodyAction>>,
    leave_visitors: HashMap<String, Vec<BodyAction>>,
    last_tokenizer_buffer: Vec<u8>,
    current_buffer: BufferLink,
}

impl FilterBodyAction {
    pub fn filter(&self, mut input: Vec<u8>) -> Vec<u8> {
        let mut buffer: Vec<u8> = self.last_tokenizer_buffer;
        buffer.append(&mut input);
        let sink = io::sink();
        let opts = TokenizerOpts::default();
        let tokenizer = Tokenizer::new(sink, opts);

        return Vec::new();
    }
}
