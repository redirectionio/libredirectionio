use crate::html::TokenType::{CommentToken, DoctypeToken, EndTagToken, ErrorToken, SelfClosingTagToken, StartTagToken, TextToken};
use std::io::Read;
use std::str;
use std::string::ToString;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    NoneToken,
    ErrorToken,
    TextToken,
    StartTagToken,
    EndTagToken,
    SelfClosingTagToken,
    CommentToken,
    DoctypeToken,
}

pub struct Error {
    pub kind: ErrorKind,
    pub read_error: Option<std::io::Error>,
}

pub enum ErrorKind {
    ReadError,
    MaxBufferError,
    EOFError,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub namespace: String,
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub data: Option<String>,
    pub attrs: Vec<Attribute>,
}

#[derive(Debug, Clone)]
struct Span {
    start: usize,
    end: usize,
}

pub struct Tokenizer<'t> {
    reader: &'t mut dyn Read,
    token: TokenType,
    err: Option<Error>,
    raw: Span,
    buffer: Vec<u8>,
    max_buffer: usize,
    data: Span,
    pending_attribute: [Span; 2],
    attribute: Vec<[Span; 2]>,
    number_attribute_returned: usize,
    raw_tag: String,
    text_is_raw: bool,
    convert_null: bool,
    allow_cdata: bool,
}

impl Token {
    fn tag_string(&self) -> String {
        if self.data.is_none() {
            return "".to_string();
        }

        let mut tag = self.data.as_ref().unwrap().clone();

        if self.attrs.is_empty() {
            return tag;
        }

        for attr in &self.attrs {
            tag.push(' ');
            tag.push_str(attr.key.as_ref().unwrap().as_str());
            tag.push_str("=\"");
            tag.push_str(attr.value.as_ref().unwrap().as_str());
            tag.push('"');
        }

        tag
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self.token_type {
            ErrorToken => "".to_string(),
            TextToken => self.data.as_ref().unwrap().clone(),
            StartTagToken => ["<", self.tag_string().as_str(), ">"].join(""),
            EndTagToken => ["</", self.tag_string().as_str(), ">"].join(""),
            SelfClosingTagToken => ["<", self.tag_string().as_str(), "/>"].join(""),
            CommentToken => ["<!--", self.data.as_ref().unwrap().as_str(), "-->"].join(""),
            DoctypeToken => ["<!DOCTYPE ", self.data.as_ref().unwrap().as_str(), ">"].join(""),
            _ => "invalid".to_string(),
        }
    }
}

impl<'t> Tokenizer<'t> {
    pub fn new(reader: &'t mut dyn Read) -> Tokenizer {
        Tokenizer::new_fragment(reader, "".to_string())
    }

    pub fn new_fragment(reader: &'t mut dyn Read, mut context_tag: String) -> Tokenizer {
        let mut tokenizer = Tokenizer {
            reader,
            token: TokenType::NoneToken,
            err: None,
            raw: Span { start: 0, end: 0 },
            buffer: Vec::new(),
            max_buffer: 0,
            data: Span { start: 0, end: 0 },
            pending_attribute: [Span { start: 0, end: 0 }, Span { start: 0, end: 0 }],
            attribute: Vec::new(),
            number_attribute_returned: 0,
            raw_tag: "".to_string(),
            text_is_raw: false,
            convert_null: false,
            allow_cdata: true,
        };

        if !context_tag.is_empty() {
            context_tag = context_tag.to_lowercase();

            match context_tag.as_str() {
                "iframe" | "noembed" | "noframes" | "noscript" | "plaintext" | "script" | "style" | "title" | "textarea" | "xmp" => {
                    tokenizer.raw_tag = context_tag.clone();
                }
                _ => {}
            }
        }

        tokenizer
    }

    pub fn err(&self) -> Option<&Error> {
        self.err.as_ref()
    }

    pub fn allow_cdata(&mut self, allow_cdata: bool) {
        self.allow_cdata = allow_cdata;
    }

    pub fn buffered(&self) -> String {
        String::from_utf8(self.buffer[self.raw.end..].to_vec()).expect("Canno create utf8 string")
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> TokenType {
        self.raw.start = self.raw.end;
        self.data.start = self.raw.end;
        self.data.end = self.raw.end;

        if self.err.is_some() {
            self.token = ErrorToken;

            return self.token;
        }

        if !self.raw_tag.is_empty() {
            if self.raw_tag == "plaintext" {
                while self.err.is_none() {
                    self.read_byte();
                }

                self.data.end = self.raw.end;
                self.text_is_raw = true;
            } else {
                self.read_raw_or_cdata();
            }

            if self.data.end > self.data.start {
                self.token = TextToken;
                self.convert_null = true;

                return self.token;
            }
        }

        self.text_is_raw = false;
        self.convert_null = false;

        'main: loop {
            let mut byte = self.read_byte() as char;

            if self.err.is_some() {
                break 'main;
            }

            if byte != '<' {
                continue 'main;
            }

            byte = self.read_byte() as char;

            if self.err.is_some() {
                break 'main;
            }

            let token_type: TokenType;

            if ('a'..='z').contains(&byte) || ('A'..='Z').contains(&byte) {
                token_type = StartTagToken;
            } else if byte == '/' {
                token_type = EndTagToken;
            } else if byte == '!' || byte == '?' {
                token_type = CommentToken;
            } else {
                self.raw.end -= 1;
                continue;
            }

            let x = self.raw.end - "<a".len();

            if self.raw.start < x {
                self.raw.end = x;
                self.data.end = x;

                self.token = TextToken;

                return self.token;
            }

            match token_type {
                StartTagToken => {
                    self.token = self.read_start_tag();

                    return self.token;
                }
                EndTagToken => {
                    let end_byte = self.read_byte() as char;

                    if self.err.is_some() {
                        break 'main;
                    }

                    if end_byte == '>' {
                        self.token = CommentToken;

                        return self.token;
                    }

                    if ('a'..='z').contains(&end_byte) || ('A'..='Z').contains(&end_byte) {
                        self.read_tag(false);

                        if self.err.is_some() {
                            self.token = ErrorToken;
                        } else {
                            self.token = EndTagToken;
                        }

                        return self.token;
                    }

                    self.raw.end -= 1;
                    self.read_until_close_angle();
                    self.token = CommentToken;

                    return self.token;
                }
                CommentToken => {
                    if byte == '!' {
                        self.token = self.read_markup_declaration();

                        return self.token;
                    }

                    self.raw.end -= 1;
                    self.read_until_close_angle();
                    self.token = CommentToken;

                    return self.token;
                }
                _ => {}
            }
        }

        if self.raw.start < self.raw.end {
            self.data.end = self.raw.end;
            self.token = TextToken;

            return self.token;
        }

        self.token = ErrorToken;

        self.token
    }

    pub fn raw(&self) -> String {
        String::from_utf8(self.buffer[self.raw.start..self.raw.end].to_vec()).expect("Cannot create utf8 string")
    }

    pub fn text(&mut self) -> Option<String> {
        match self.token {
            TextToken | CommentToken | DoctypeToken => {
                let mut s = str::from_utf8(&self.buffer[self.data.start..self.data.end])
                    .expect("Cannot create utf8 string")
                    .to_string();

                self.data.start = self.raw.end;
                self.data.end = self.raw.end;

                if self.convert_null || self.token == TextToken && s.contains('\x00') {
                    s = s.replace('\x00', "\u{fffd}".to_string().as_str());
                }

                //                if !self.text_is_raw {
                //                    s = escape(s)
                //                }

                Some(s)
            }
            _ => None,
        }
    }

    pub fn tag_name(&mut self) -> (Option<String>, bool) {
        if self.data.start < self.data.end {
            match self.token {
                StartTagToken | EndTagToken | SelfClosingTagToken => {
                    let s = str::from_utf8(&self.buffer[self.data.start..self.data.end])
                        .expect("Cannot create utf8 string")
                        .to_string();

                    self.data.start = self.raw.end;
                    self.data.end = self.raw.end;

                    return (Some(s.to_lowercase()), self.number_attribute_returned < self.attribute.len());
                }
                _ => {}
            }
        }

        (None, false)
    }

    pub fn tag_attr(&mut self) -> (Option<String>, Option<String>, bool) {
        if self.number_attribute_returned < self.attribute.len() {
            match self.token {
                StartTagToken | SelfClosingTagToken => {
                    let attr = &self.attribute[self.number_attribute_returned];
                    self.number_attribute_returned += 1;

                    let key = str::from_utf8(&self.buffer[attr[0].start..attr[0].end])
                        .expect("Cannot create utf8 string")
                        .to_string();
                    let val = str::from_utf8(&self.buffer[attr[1].start..attr[1].end])
                        .expect("Cannot create utf8 string")
                        .to_string();

                    return (
                        Some(key.to_lowercase()),
                        Some(val),
                        self.number_attribute_returned < self.attribute.len(),
                    );
                }
                _ => {}
            }
        }

        (None, None, false)
    }

    pub fn token(&mut self) -> Token {
        let mut token = Token {
            token_type: self.token,
            attrs: Vec::new(),
            data: None,
        };

        match self.token {
            TextToken | CommentToken | DoctypeToken => {
                token.data = self.text();
            }
            StartTagToken | SelfClosingTagToken | EndTagToken => {
                let (name, mut has_attr) = self.tag_name();

                while has_attr {
                    let (key, val, has_attr_attr) = self.tag_attr();
                    has_attr = has_attr_attr;

                    token.attrs.push(Attribute {
                        key,
                        value: val,
                        namespace: "".to_string(),
                    })
                }

                token.data = name;
            }
            _ => {}
        }

        token
    }

    pub fn set_max_buffer(&mut self, max_buffer: usize) {
        self.max_buffer = max_buffer;
    }

    fn read_byte(&mut self) -> u8 {
        if self.raw.end >= self.buffer.len() {
            //            let new_buffer= self.buffer[self.raw.start..self.raw.end].to_vec().clone();
            //            let start = self.raw.start;
            //
            //            if start != 0 {
            //                self.data.start -= start;
            //                self.data.end -= start;
            //                self.pending_attribute[0].start -= start;
            //                self.pending_attribute[0].end -= start;
            //                self.pending_attribute[1].start -= start;
            //                self.pending_attribute[1].end -= start;
            //
            //                for attribute in &mut self.attribute {
            //                    attribute[0].start -= start;
            //                    attribute[0].end -= start;
            //                    attribute[1].start -= start;
            //                    attribute[1].end -= start;
            //                }
            //            }

            let mut new_byte_buffer = Vec::new();
            let error = self.reader.read_to_end(new_byte_buffer.as_mut());

            if error.is_err() {
                self.err = Some(Error {
                    kind: ErrorKind::ReadError,
                    read_error: error.err(),
                });

                return 0;
            }

            if new_byte_buffer.is_empty() {
                self.err = Some(Error {
                    kind: ErrorKind::EOFError,
                    read_error: None,
                });

                return 0;
            }

            self.buffer.append(&mut new_byte_buffer);
        }

        let byte = self.buffer[self.raw.end];

        self.raw.end += 1;

        if self.max_buffer > 0 && self.raw.end - self.raw.start >= self.max_buffer {
            self.err = Some(Error {
                kind: ErrorKind::MaxBufferError,
                read_error: None,
            });

            return 0;
        }

        byte
    }

    fn skip_white_space(&mut self) {
        if self.err.is_some() {
            return;
        }

        loop {
            let byte = self.read_byte() as char;

            if self.err.is_some() {
                return;
            }

            match byte {
                ' ' | '\n' | '\r' | '\t' | '\x0c' => {}
                _ => {
                    self.raw.end -= 1;

                    return;
                }
            }
        }
    }

    fn read_raw_or_cdata(&mut self) {
        if self.raw_tag == "script" {
            self.read_script();
            self.text_is_raw = true;
            self.raw_tag = "".to_string();

            return;
        }

        loop {
            let mut byte = self.read_byte() as char;

            if self.err.is_some() {
                break;
            }

            if byte != '<' {
                continue;
            }

            byte = self.read_byte() as char;

            if self.err.is_some() {
                break;
            }

            if byte != '/' {
                continue;
            }

            if self.read_raw_end_tag() || self.err.is_some() {
                break;
            }
        }

        self.data.end = self.raw.end;
        self.text_is_raw = self.raw_tag != "textarea" && self.raw_tag != "title";
        self.raw_tag = "".to_string();
    }

    fn read_raw_end_tag(&mut self) -> bool {
        for i in 0..self.raw_tag.len() {
            let byte = self.read_byte();

            if self.err.is_some() {
                return false;
            }

            if byte != self.raw_tag.as_bytes()[i] && byte != self.raw_tag.as_bytes()[i] - (b'a' - b'A') {
                self.raw.end -= 1;

                return false;
            }
        }

        let byte = self.read_byte() as char;

        if self.err.is_some() {
            return false;
        }

        match byte {
            ' ' | '\n' | '\r' | '\t' | '\x0c' | '/' | '>' => {
                self.raw.end -= 3 + self.raw_tag.len();

                true
            }
            _ => {
                self.raw.end -= 1;

                false
            }
        }
    }

    fn read_script(&mut self) {
        self.read_script_data();
        self.data.end = self.raw.end;
    }

    fn read_script_data(&mut self) {
        let byte = self.read_byte() as char;

        if self.err.is_some() {
            return;
        }

        if byte == '<' {
            self.read_script_data_less_than_sign();

            return;
        }

        self.read_script_data();
    }

    fn read_script_data_less_than_sign(&mut self) {
        let byte = self.read_byte() as char;

        if self.err.is_some() {
            return;
        }

        match byte {
            '/' => {
                self.read_script_data_end_tag_open();
            }
            '!' => {
                self.read_script_data_escape_start();
            }
            _ => {
                self.raw.end -= 1;
                self.read_script_data();
            }
        }
    }

    fn read_script_data_end_tag_open(&mut self) {
        if self.read_raw_end_tag() || self.err.is_some() {
            return;
        }

        self.read_script_data();
    }

    fn read_script_data_escape_start(&mut self) {
        let byte = self.read_byte() as char;

        if self.err.is_some() {
            return;
        }

        if byte == '-' {
            self.read_script_data_escape_start_dash();

            return;
        }

        self.raw.end -= 1;
        self.read_script_data();
    }

    fn read_script_data_escape_start_dash(&mut self) {
        let byte = self.read_byte() as char;

        if self.err.is_some() {
            return;
        }

        if byte == '-' {
            self.read_script_data_escaped_dash_dash();

            return;
        }

        self.raw.end -= 1;
        self.read_script_data();
    }

    fn read_script_data_escaped(&mut self) {
        let byte = self.read_byte() as char;

        if self.err.is_some() {
            return;
        }

        match byte {
            '-' => {
                self.read_script_data_escaped_dash();
            }
            '<' => {
                self.read_script_data_escaped_less_than_sign();
            }
            _ => {
                self.read_script_data_escaped();
            }
        }
    }

    fn read_script_data_escaped_dash(&mut self) {
        let byte = self.read_byte() as char;

        if self.err.is_some() {
            return;
        }

        match byte {
            '-' => {
                self.read_script_data_escaped_dash_dash();
            }
            '<' => {
                self.read_script_data_escaped_less_than_sign();
            }
            _ => {
                self.read_script_data_escaped();
            }
        }
    }

    fn read_script_data_escaped_dash_dash(&mut self) {
        let byte = self.read_byte() as char;

        if self.err.is_some() {
            return;
        }

        match byte {
            '-' => {
                self.read_script_data_escaped_dash_dash();
            }
            '<' => {
                self.read_script_data_escaped_less_than_sign();
            }
            '>' => {
                self.read_script_data();
            }
            _ => {
                self.read_script_data_escaped();
            }
        }
    }

    fn read_script_data_escaped_less_than_sign(&mut self) {
        let byte = self.read_byte() as char;

        if self.err.is_some() {
            return;
        }

        if byte == '/' {
            self.read_script_data_escaped_end_tag_open();

            return;
        }

        if ('a'..='z').contains(&byte) || ('A'..='Z').contains(&byte) {
            self.read_script_data_double_escape_start();

            return;
        }

        self.raw.end -= 1;
        self.read_script_data();
    }

    fn read_script_data_escaped_end_tag_open(&mut self) {
        if self.read_raw_end_tag() || self.err.is_some() {
            return;
        }

        self.read_script_data_escaped();
    }

    fn read_script_data_double_escape_start(&mut self) {
        self.raw.end -= 1;

        for i in 0.."script".len() {
            let byte = self.read_byte();

            if self.err.is_some() {
                return;
            }

            if byte != b"script"[i] && byte != b"SCRIPT"[i] {
                self.raw.end -= 1;
                self.read_script_data_escaped();

                return;
            }
        }

        let byte = self.read_byte() as char;

        if self.err.is_some() {
            return;
        }

        match byte {
            ' ' | '\n' | '\r' | '\t' | '\x0c' | '/' | '>' => {
                self.read_script_data_double_escaped();
            }
            _ => {
                self.raw.end -= 1;
                self.read_script_data_escaped();
            }
        }
    }

    fn read_script_data_double_escaped(&mut self) {
        let byte = self.read_byte() as char;

        if self.err.is_some() {
            return;
        }

        match byte {
            '-' => {
                self.read_script_data_double_escaped_dash();
            }
            '<' => {
                self.read_script_data_double_escaped_less_than_sign();
            }
            _ => {
                self.read_script_data_double_escaped();
            }
        }
    }

    fn read_script_data_double_escaped_dash(&mut self) {
        let byte = self.read_byte() as char;

        if self.err.is_some() {
            return;
        }

        match byte {
            '-' => {
                self.read_script_data_double_escaped_dash_dash();
            }
            '<' => {
                self.read_script_data_double_escaped_less_than_sign();
            }
            _ => {
                self.read_script_data_double_escaped();
            }
        }
    }

    fn read_script_data_double_escaped_dash_dash(&mut self) {
        let byte = self.read_byte() as char;

        if self.err.is_some() {
            return;
        }

        match byte {
            '-' => {
                self.read_script_data_double_escaped_dash_dash();
            }
            '<' => {
                self.read_script_data_double_escaped_less_than_sign();
            }
            '>' => {
                self.read_script_data();
            }
            _ => {
                self.read_script_data_double_escaped();
            }
        }
    }

    fn read_script_data_double_escaped_less_than_sign(&mut self) {
        let byte = self.read_byte() as char;

        if self.err.is_some() {
            return;
        }

        if byte == '/' {
            self.read_script_data_double_escaped_end();

            return;
        }

        self.raw.end -= 1;
        self.read_script_data_double_escaped();
    }

    fn read_script_data_double_escaped_end(&mut self) {
        if self.read_raw_end_tag() {
            self.raw.end += "</script>".len();
            self.read_script_data_escaped();

            return;
        }

        if self.err.is_some() {
            return;
        }

        self.read_script_data_double_escaped();
    }

    fn read_comment(&mut self) {
        self.data.start = self.raw.end;
        let mut dash_count = 2;

        loop {
            let byte = self.read_byte() as char;

            if self.err.is_some() {
                if dash_count > 2 {
                    dash_count = 2;
                }

                self.data.end = self.raw.end - dash_count;

                break;
            }

            match byte {
                '-' => {
                    dash_count += 1;

                    continue;
                }
                '>' => {
                    if dash_count >= 2 {
                        self.data.end = self.raw.end - "-->".len();

                        break;
                    }
                }
                '!' => {
                    if dash_count >= 2 {
                        let byte = self.read_byte() as char;

                        if self.err.is_some() {
                            self.data.end = self.raw.end;

                            break;
                        }

                        if byte == '>' {
                            self.data.end = self.raw.end - "--!>".len();

                            break;
                        }
                    }
                }
                _ => {}
            }

            dash_count = 0;
        }

        if self.data.end < self.data.start {
            self.data.end = self.data.start;
        }
    }

    fn read_until_close_angle(&mut self) {
        self.data.start = self.raw.end;

        loop {
            let byte = self.read_byte() as char;

            if self.err.is_some() {
                self.data.end = self.raw.end;

                return;
            }

            if byte == '>' {
                self.data.end = self.raw.end - ">".len();

                return;
            }
        }
    }

    fn read_markup_declaration(&mut self) -> TokenType {
        self.data.start = self.raw.end;
        let first_byte = self.read_byte() as char;

        if self.err.is_some() {
            self.data.end = self.raw.end;

            return CommentToken;
        }

        let second_byte = self.read_byte() as char;

        if self.err.is_some() {
            self.data.end = self.raw.end;

            return CommentToken;
        }

        if first_byte == '-' && second_byte == '-' {
            self.read_comment();

            return CommentToken;
        }

        self.raw.end -= 2;

        if self.read_doc_type() {
            return DoctypeToken;
        }

        if self.allow_cdata && self.read_cdata() {
            self.convert_null = true;

            return TextToken;
        }

        self.read_until_close_angle();

        CommentToken
    }

    fn read_doc_type(&mut self) -> bool {
        let doctype = "DOCTYPE".to_string();

        for i in 0..doctype.len() {
            let byte = self.read_byte();

            if self.err.is_some() {
                self.data.end = self.raw.end;

                return false;
            }

            if byte != doctype.as_bytes()[i] && byte != doctype.as_bytes()[i] + (b'a' - b'A') {
                self.raw.end = self.data.start;

                return false;
            }
        }

        self.skip_white_space();

        if self.err.is_some() {
            self.data.start = self.raw.end;
            self.data.end = self.raw.end;

            return true;
        }

        self.read_until_close_angle();

        true
    }

    fn read_cdata(&mut self) -> bool {
        let cdata = "[CDATA[".to_string();

        for i in 0..cdata.len() {
            let byte = self.read_byte();

            if self.err.is_some() {
                self.data.end = self.raw.end;

                return false;
            }

            if byte != cdata.as_bytes()[i] {
                self.raw.end = self.data.start;

                return false;
            }
        }

        self.data.start = self.raw.end;
        let mut brackets = 0;

        loop {
            let byte = self.read_byte() as char;

            if self.err.is_some() {
                self.data.end = self.raw.end;

                return true;
            }

            match byte {
                ']' => {
                    brackets += 1;
                }
                '>' => {
                    if brackets > 2 {
                        self.data.end = self.raw.end - "]]>".len();

                        return true;
                    }

                    brackets = 0;
                }
                _ => {
                    brackets = 0;
                }
            }
        }
    }

    fn start_tag_in(&self, ss: Vec<String>) -> bool {
        'main: for s in ss {
            if self.data.end - self.data.start != s.len() {
                continue;
            }

            for i in 0..s.len() {
                let mut c = self.buffer[self.data.start + i];

                if (b'A'..=b'Z').contains(&c) {
                    c += b'a' - b'A';
                }

                if c != s.as_bytes()[i] {
                    continue 'main;
                }
            }

            return true;
        }

        false
    }

    fn read_start_tag(&mut self) -> TokenType {
        self.read_tag(true);

        if self.err.is_some() {
            return ErrorToken;
        }

        let mut raw = false;
        let mut byte = self.buffer[self.data.start] as u8;

        if (b'A'..=b'Z').contains(&byte) {
            byte += b'a' - b'A';
        }

        let byte_char = byte as char;

        match byte_char {
            'i' => {
                raw = self.start_tag_in(vec!["iframe".to_string()]);
            }
            'n' => {
                raw = self.start_tag_in(vec!["noembed".to_string(), "noframes".to_string(), "noscript".to_string()]);
            }
            'p' => {
                raw = self.start_tag_in(vec!["plaintext".to_string()]);
            }
            's' => {
                raw = self.start_tag_in(vec!["script".to_string(), "style".to_string()]);
            }
            't' => {
                raw = self.start_tag_in(vec!["textarea".to_string(), "title".to_string()]);
            }
            'x' => {
                raw = self.start_tag_in(vec!["xmp".to_string()]);
            }
            _ => {}
        }

        if raw {
            self.raw_tag = str::from_utf8(&self.buffer[self.data.start..self.data.end])
                .expect("Cannot create utf8 string")
                .to_string()
                .to_lowercase();
        }

        if self.err.is_none() && self.buffer[self.raw.end - 2] == b'/' {
            return SelfClosingTagToken;
        }

        StartTagToken
    }

    fn read_tag(&mut self, save_attr: bool) {
        self.attribute = self.attribute[..0].to_vec();
        self.number_attribute_returned = 0;
        self.read_tag_name();
        self.skip_white_space();

        if self.err.is_some() {
            return;
        }

        loop {
            let byte = self.read_byte() as char;

            if self.err.is_some() || byte == '>' {
                return;
            }

            self.raw.end -= 1;
            self.read_tag_name_attr_key();
            self.read_tag_name_attr_value();

            if save_attr && self.pending_attribute[0].start != self.pending_attribute[0].end {
                self.attribute.push(self.pending_attribute.clone());
            }

            self.skip_white_space();

            if self.err.is_some() {
                return;
            }
        }
    }

    fn read_tag_name(&mut self) {
        self.data.start = self.raw.end - 1;

        loop {
            let byte = self.read_byte() as char;

            if self.err.is_some() {
                self.data.end = self.raw.end;

                return;
            }

            match byte {
                ' ' | '\n' | '\r' | '\t' | '\x0c' => {
                    self.data.end = self.raw.end - 1;

                    return;
                }
                '/' | '>' => {
                    self.raw.end -= 1;
                    self.data.end = self.raw.end;

                    return;
                }
                _ => {}
            }
        }
    }

    fn read_tag_name_attr_key(&mut self) {
        self.pending_attribute[0].start = self.raw.end;

        loop {
            let byte = self.read_byte() as char;

            if self.err.is_some() {
                self.pending_attribute[0].end = self.raw.end;

                return;
            }

            match byte {
                ' ' | '\n' | '\r' | '\t' | '\x0c' | '/' => {
                    self.pending_attribute[0].end = self.raw.end - 1;

                    return;
                }
                '=' | '>' => {
                    self.raw.end -= 1;
                    self.pending_attribute[0].end = self.raw.end;

                    return;
                }
                _ => {}
            }
        }
    }

    fn read_tag_name_attr_value(&mut self) {
        self.pending_attribute[1].start = self.raw.end;
        self.pending_attribute[1].end = self.raw.end;
        self.skip_white_space();

        if self.err.is_some() {
            return;
        }

        let byte = self.read_byte() as char;

        if self.err.is_some() {
            return;
        }

        if byte != '=' {
            self.raw.end -= 1;

            return;
        }

        self.skip_white_space();

        if self.err.is_some() {
            return;
        }

        let quote = self.read_byte() as char;

        if self.err.is_some() {
            return;
        }

        match quote {
            '>' => {
                self.raw.end -= 1;
            }
            '\'' | '"' => {
                self.pending_attribute[1].start = self.raw.end;

                loop {
                    let byte = self.read_byte() as char;

                    if self.err.is_some() {
                        self.pending_attribute[1].end = self.raw.end;

                        return;
                    }

                    if byte == quote {
                        self.pending_attribute[1].end = self.raw.end - 1;

                        return;
                    }
                }
            }
            _ => {
                self.pending_attribute[1].start = self.raw.end - 1;

                loop {
                    let byte = self.read_byte() as char;

                    if self.err.is_some() {
                        self.pending_attribute[1].end = self.raw.end;

                        return;
                    }

                    match byte {
                        ' ' | '\n' | '\r' | '\t' | '\x0c' => {
                            self.pending_attribute[1].end = self.raw.end - 1;

                            return;
                        }
                        '>' => {
                            self.raw.end -= 1;
                            self.pending_attribute[1].end = self.raw.end;

                            return;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    macro_rules! html_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (html, golden) = $value;
                let reader = &mut html.as_bytes() as &mut dyn std::io::Read;
                let mut tokenizer = Tokenizer::new(reader);

                if !golden.is_empty() {
                    let splits = golden.split("$");

                    for split in splits {
                        let token_type = tokenizer.next();

                        assert_ne!(token_type, ErrorToken);
                        let actual_token = tokenizer.token();
                        assert_eq!(actual_token.to_string(), split);
                    }
                }

                tokenizer.next();
                assert_eq!(true, tokenizer.err().is_some());
            }
        )*
        }
    }

    use super::*;

    html_tests! {
        empty: ("".to_string(), "".to_string()),
        text: ("foo  bar".to_string(), "foo  bar".to_string()),
        entity: ("one &lt; two".to_string(), "one &lt; two".to_string()),
        tags: ("<a>b<c/>d</e>".to_string(), "<a>$b$<c/>$d$</e>".to_string()),
        not_a_tag_0: ("<".to_string(), "<".to_string()),
        not_a_tag_1: ("</".to_string(), "</".to_string()),
        not_a_tag_2: ("</>".to_string(), "<!---->".to_string()),
        not_a_tag_3: ("a</>b".to_string(), "a$<!---->$b".to_string()),
        not_a_tag_4: ("</ >".to_string(), "<!-- -->".to_string()),
        not_a_tag_5: ("</.".to_string(), "<!--.-->".to_string()),
        not_a_tag_6: ("</.>".to_string(), "<!--.-->".to_string()),
        not_a_tag_7: ("a < b".to_string(), "a < b".to_string()),
        not_a_tag_8: ("<.>".to_string(), "<.>".to_string()),
        not_a_tag_9: ("a<<<b>>>c".to_string(), "a<<$<b>$>>c".to_string()),
        not_a_tag_10: ("i x<0 and y < 0 then x*y>0".to_string(), "i x<0 and y < 0 then x*y>0".to_string()),
        not_a_tag_11: ("<<p>".to_string(), "<$<p>".to_string()),
        tag_name_eof_0: ("<a".to_string(), "".to_string()),
        tag_name_eof_1: ("<a ".to_string(), "".to_string()),
        tag_name_eof_2: ("a<b".to_string(), "a".to_string()),
        tag_name_eof_3: ("<a><b ".to_string(), "<a>".to_string()),
        tag_name_eof_4: ("<a x ".to_string(), "".to_string()),
        malformed_tag_0: ("<p</p>".to_string(), "<p< p=\"\">".to_string()),
        malformed_tag_1: ("<p </p>".to_string(), "<p <=\"\" p=\"\">".to_string()),
        malformed_tag_2: ("<p id".to_string(), "".to_string()),
        malformed_tag_3: ("<p id=".to_string(), "".to_string()),
        malformed_tag_4: ("<p id=>".to_string(), "<p id=\"\">".to_string()),
        malformed_tag_5: ("<p id=0".to_string(), "".to_string()),
        malformed_tag_6: ("<p id=0</p>".to_string(), "<p id=\"0</p\">".to_string()),
        malformed_tag_7: ("<p id=\"0</p>".to_string(), "".to_string()),
        malformed_tag_8: ("<p id=\"0\"</p>".to_string(), "<p id=\"0\" <=\"\" p=\"\">".to_string()),
        malformed_tag_9: ("<p></p id".to_string(), "<p>".to_string()),
        basic_raw_text: ("<script><a></b></script>".to_string(), "<script>$<a></b>$</script>".to_string()),
        unfinished_script_end_tag: ("<SCRIPT>a</SCR".to_string(), "<script>$a</SCR".to_string()),
        broken_script_end_tag: ("<SCRIPT>a</SCR ipt>".to_string(), "<script>$a</SCR ipt>".to_string()),
        eof_in_script_end_tag: ("<SCRIPT>a</SCRipt".to_string(), "<script>$a</SCRipt".to_string()),
        scriptx_end_tag: ("<SCRIPT>a</SCRiptx".to_string(), "<script>$a</SCRiptx".to_string()),
        space_completes_script_end_tag: ("<SCRIPT>a</SCRipt ".to_string(), "<script>$a".to_string()),
        sup_completes_script_end_tag: ("<SCRIPT>a</SCRipt>".to_string(), "<script>$a$</script>".to_string()),
        nested_script_tag: ("<SCRIPT>a</SCRipt<script>".to_string(), "<script>$a</SCRipt<script>".to_string()),
        script_end_tag_after_unfinihsed: ("<SCRIPT>a</SCRipt</script>".to_string(), "<script>$a</SCRipt$</script>".to_string()),
        script_style_mistmatched_tag: ("<script>a</style>".to_string(), "<script>$a</style>".to_string()),
        style_element_with_entity: ("<style>&apos;".to_string(), "<style>$&apos;".to_string()),
        textarea_with_tag: ("<textarea><div></textarea>".to_string(), "<textarea>$<div>$</textarea>".to_string()),
        title_with_tag_and_entity: ("<title><b>K&amp;R C</b></title>".to_string(), "<title>$<b>K&amp;R C</b>$</title>".to_string()),
        proper_doctype: ("<!DOCTYPE html>".to_string(), "<!DOCTYPE html>".to_string()),
        doctype_with_no_space: ("<!doctypehtml>".to_string(), "<!DOCTYPE html>".to_string()),
        doctype_with_two_space: ("<!doctype  html>".to_string(), "<!DOCTYPE html>".to_string()),
        doctype_looks_like: ("<!DOCUMENT html>".to_string(), "<!--DOCUMENT html-->".to_string()),
        doctype_at_eof: ("<!DOCTYPE".to_string(), "<!DOCTYPE >".to_string()),
        xml_processing_instruction: ("<?xml?>".to_string(), "<!--?xml?-->".to_string()),
        comment_0: ("abc<b><!-- skipme --></b>def".to_string(), "abc$<b>$<!-- skipme -->$</b>$def".to_string()),
        comment_1: ("a<!-->z".to_string(), "a$<!---->$z".to_string()),
        comment_2: ("a<!--->z".to_string(), "a$<!---->$z".to_string()),
        comment_3: ("a<!--x>-->z".to_string(), "a$<!--x>-->$z".to_string()),
        comment_4: ("a<!--x->-->z".to_string(), "a$<!--x->-->$z".to_string()),
        comment_5: ("a<!>z".to_string(), "a$<!---->$z".to_string()),
        comment_6: ("a<!->z".to_string(), "a$<!----->$z".to_string()),
        comment_7: ("a<!---<>z".to_string(), "a$<!---<>z-->".to_string()),
        comment_8: ("a<!--z".to_string(), "a$<!--z-->".to_string()),
        comment_9: ("a<!--z-".to_string(), "a$<!--z-->".to_string()),
        comment_10: ("a<!--z--".to_string(), "a$<!--z-->".to_string()),
        comment_11: ("a<!--z---".to_string(), "a$<!--z--->".to_string()),
        comment_12: ("a<!--z----".to_string(), "a$<!--z---->".to_string()),
        comment_13: ("a<!--x--!>z".to_string(), "a$<!--x-->$z".to_string()),
        backslash: ("<p id=\"a\\\"b\">".to_string(), "<p id=\"a\\\" b\"=\"\">".to_string()),
        tricky: ("<p \t\n iD=\"a&quot;B\"  foo=\"bar\"><EM>te&lt;&amp;;xt</em></p>".to_string(), "<p id=\"a&quot;B\" foo=\"bar\">$<em>$te&lt;&amp;;xt$</em>$</p>".to_string()),
        no_such_entity: ("<a b=\"c&noSuchEntity;d\">&lt;&alsoDoesntExist;&".to_string(), "<a b=\"c&noSuchEntity;d\">$&lt;&alsoDoesntExist;&".to_string()),
        entity_without_semicolon: ("&notit;&notin;<a b=\"q=z&amp=5&notice=hello&not;=world\">".to_string(), "&notit;&notin;$<a b=\"q=z&amp=5&notice=hello&not;=world\">".to_string()),
        attribute_empty: ("<input disabled FOO>".to_string(), "<input disabled=\"\" foo=\"\">".to_string()),
        attribute_empty_with_space: ("<input disabled FOO >".to_string(), "<input disabled=\"\" foo=\"\">".to_string()),
        attribute_unquoted: ("<input value=yes FOO=BAR>".to_string(), "<input value=\"yes\" foo=\"BAR\">".to_string()),
        attribute_unquoted_with_space: ("<input value = yes FOO = BAR>".to_string(), "<input value=\"yes\" foo=\"BAR\">".to_string()),
        attribute_unquoted_with_trailing_space: ("<input value=yes FOO=BAR >".to_string(), "<input value=\"yes\" foo=\"BAR\">".to_string()),
        attribute_value_single_quoted: ("<input value='yes' FOO='BAR'>".to_string(), "<input value=\"yes\" foo=\"BAR\">".to_string()),
        attribute_value_single_quoted_with_trailing_space: ("<input value='yes' FOO='BAR' >".to_string(), "<input value=\"yes\" foo=\"BAR\">".to_string()),
        attribute_value_double_quoted: ("<input value=\"I'm an attribute\" FOO=\"BAR\">".to_string(), "<input value=\"I'm an attribute\" foo=\"BAR\">".to_string()),
        attribute_name_characters: ("<meta http-equiv=\"content-type\">".to_string(), "<meta http-equiv=\"content-type\">".to_string()),
        attribute_mixed: ("a<P V=\"0 1\" w='2' X=3 y>z".to_string(), "a$<p v=\"0 1\" w=\"2\" x=\"3\" y=\"\">$z".to_string()),
        attribute_with_a_solitary_single_quote: ("<p id=can't><p id=won't>".to_string(), "<p id=\"can't\">$<p id=\"won't\">".to_string()),
    }
}
