use std::io::{Read, ErrorKind};
use std::io::Error;
use std::str;
use std::string::ToString;
use crate::html::TokenType::{CommentToken, DoctypeToken, TextToken, ErrorToken, StartTagToken, SelfClosingTagToken, EndTagToken};

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

struct Tokenizer<'t> {
    reader: &'t mut Read,
    token: TokenType,
    err: Option<Error>,
    read_error: Option<Error>,
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

        if self.attrs.len() == 0 {
            return tag;
        }

        for attr in &self.attrs {
            tag.push_str(" ");
            tag.push_str(attr.key.as_ref().unwrap().as_str());
            tag.push_str("=\"");
            tag.push_str(attr.value.as_ref().unwrap().as_str());
            tag.push_str("\"");
        }

        return tag;
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self.token_type {
            ErrorToken => {
                "".to_string()
            }
            TextToken => {
                self.data.as_ref().unwrap().clone()
            }
            StartTagToken => {
                ["<", self.tag_string().as_str(), ">"].join("")
            }
            EndTagToken => {
                ["</", self.tag_string().as_str(), ">"].join("")
            }
            SelfClosingTagToken => {
                ["<", self.tag_string().as_str(), "/>"].join("")
            }
            CommentToken => {
                ["<!--", self.data.as_ref().unwrap().as_str(), "-->"].join("")
            }
            DoctypeToken => {
                ["<!DOCTYPE ",  self.data.as_ref().unwrap().as_str(), ">"].join("")
            }
            _ => {
                "invalid".to_string()
            }
        }
    }
}

impl<'t> Tokenizer<'t> {
    pub fn new(reader: &'t mut Read) -> Tokenizer {
        return Tokenizer::new_fragment(reader, "".to_string());
    }

    pub fn new_fragment(reader: &'t mut Read, mut context_tag: String) -> Tokenizer {
        let mut tokenizer = Tokenizer {
            reader,
            token: TokenType::NoneToken,
            err: None,
            read_error: None,
            raw: Span {
                start: 0,
                end: 0,
            },
            buffer: Vec::new(),
            max_buffer: 0,
            data: Span {
                start: 0,
                end: 0,
            },
            pending_attribute: [
                Span { start: 0, end: 0 },
                Span { start: 0, end: 0 },
            ],
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

        return tokenizer;
    }

    pub fn err(&self) -> Option<&Error> {
        return self.err.as_ref();
    }

    pub fn allow_cdata(&mut self, allow_cdata: bool) {
        self.allow_cdata = allow_cdata;
    }

    pub fn buffered(&self) -> Vec<u8> {
        return self.buffer[self.raw.end..].to_vec();
    }

    pub fn next(&mut self) -> TokenType {
        self.raw.start = self.raw.end;
        self.data.start = self.raw.end;
        self.data.end = self.raw.end;

        if self.err.is_some() {
            self.token = ErrorToken;

            return self.token;
        }

        if !self.raw_tag.is_empty() {
            if self.raw_tag == "plaintext".to_string() {
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

            let mut token_type: TokenType;

            if 'a' <= byte && byte <= 'z' || 'A' <= byte && byte <= 'Z' {
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
                },
                EndTagToken => {
                    let end_byte = self.read_byte() as char;

                    if self.err.is_some() {
                        break 'main;
                    }

                    if end_byte == '>' {
                        self.token = CommentToken;

                        return self.token;
                    }

                    if 'a' <= end_byte && end_byte <= 'z' || 'A' <= end_byte && end_byte <= 'Z' {
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
                },
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

        return self.token;
    }

    pub fn raw(&self) -> &[u8] {
        return &self.buffer[self.raw.start..self.raw.end];
    }

    pub fn text(&mut self) -> Option<String> {
        match self.token {
            TextToken | CommentToken | DoctypeToken => {
                let mut s = str::from_utf8(&self.buffer[self.data.start..self.data.end]).expect("Cannot create utf8 string").to_string();

                self.data.start = self.raw.end;
                self.data.end = self.raw.end;

                s = convert_next_lines(s);

                if self.convert_null || self.token == TextToken && s.contains('\x00') {
                    s = s.replace('\x00', "\u{fffd}".to_string().as_str());
                }

//                if !self.text_is_raw {
//                    s = escape(s)
//                }

                return Some(s);
            }
            _ => {
                return None;
            }
        }
    }

    pub fn tag_name(&mut self) -> (Option<String>, bool) {
        if self.data.start < self.data.end {
            match self.token {
                StartTagToken | EndTagToken | SelfClosingTagToken => {
                    let s = str::from_utf8(&self.buffer[self.data.start..self.data.end]).expect("Cannot create utf8 string").to_string();

                    self.data.start = self.raw.end;
                    self.data.end = self.raw.end;

                    return (Some(s.to_lowercase()), self.number_attribute_returned < self.attribute.len());
                }
                _ => {}
            }
        }

        return (None, false);
    }

    pub fn tag_attr(&mut self) -> (Option<String>, Option<String>, bool) {
        if self.number_attribute_returned < self.attribute.len() {
            match self.token {
                StartTagToken | SelfClosingTagToken => {
                    let attr = &self.attribute[self.number_attribute_returned];
                    self.number_attribute_returned += 1;

                    let key = str::from_utf8(&self.buffer[attr[0].start..attr[0].end]).expect("Cannot create utf8 string").to_string();
                    let mut val = str::from_utf8(&self.buffer[attr[1].start..attr[1].end]).expect("Cannot create utf8 string").to_string();

                    val = convert_next_lines(val);

                    return (Some(key.to_lowercase()), Some(val), self.number_attribute_returned < self.attribute.len());
                }
                _ => {}
            }
        }

        return (None, None, false);
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
            },
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
            _ => {},
        }

        return token;
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
                self.err = error.err();

                return 0;
            }

            if new_byte_buffer.len() == 0 {
                self.err = Some(Error::new(ErrorKind::UnexpectedEof, "eof"));

                return 0;
            }

            self.buffer.append(&mut new_byte_buffer);
        }

        let byte = self.buffer[self.raw.end];

        self.raw.end += 1;

        if self.max_buffer > 0 && self.raw.end - self.raw.start >= self.max_buffer {
            self.err = Some(Error::new(ErrorKind::UnexpectedEof, "max buffer reached"));

            return 0;
        }

        return byte;
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

            if byte !=  '/' {
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

            if byte != self.raw_tag.as_bytes()[i] && byte != self.raw_tag.as_bytes()[i] - ('a' as u8 - 'A' as u8) {
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

                return true;
            }
            _ => {
                self.raw.end -= 1;

                return false;
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
            },
            '!' => {
                self.read_script_data_escape_start();
            },
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
            },
            '<' => {
                self.read_script_data_escaped_less_than_sign();
            },
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
            },
            '<' => {
                self.read_script_data_escaped_less_than_sign();
            },
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
            },
            '<' => {
                self.read_script_data_escaped_less_than_sign();
            },
            '>' => {
                self.read_script_data();
            },
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

        if 'a' <= byte && byte <= 'z' || 'A' <= byte && byte <= 'Z' {
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

            if byte != "script".as_bytes()[i] && byte != "SCRIPT".as_bytes()[i] {
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
            },
            '<' => {
                self.read_script_data_double_escaped_less_than_sign();
            },
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
            },
            '<' => {
                self.read_script_data_double_escaped_less_than_sign();
            },
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
            },
            '<' => {
                self.read_script_data_double_escaped_less_than_sign();
            },
            '>' => {
                self.read_script_data();
            },
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
                _ => {

                }
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

        return CommentToken;
    }

    fn read_doc_type(&mut self) -> bool {
        let doctype = "DOCTYPE".to_string();

        for i in 0..doctype.len() {
            let byte = self.read_byte();

            if self.err.is_some() {
                self.data.end = self.raw.end;

                return false;
            }

            if byte != doctype.as_bytes()[i] && byte != doctype.as_bytes()[i] + ('a' as u8 - 'A' as u8) {
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

        return true;
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
                },
                '>' => {
                    if brackets > 2 {
                        self.data.end = self.raw.end - "]]>".len();

                        return true;
                    }

                    brackets = 0;
                },
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

                if 'A' as u8 <= c && c <= 'Z' as u8 {
                    c += 'a' as u8 - 'A' as u8;
                }

                if c != s.as_bytes()[i] {
                    continue 'main;
                }
            }

            return true;
        }

        return false;
    }

    fn read_start_tag(&mut self) -> TokenType {
        self.read_tag(true);

        if self.err.is_some() {
            return ErrorToken;
        }

        let mut raw = false;
        let mut byte = self.buffer[self.data.start] as u8;

        if 'A' as u8 <= byte && byte <= 'Z' as u8 {
            byte += 'a' as u8 - 'A' as u8;
        }

        let byte_char = byte as char;

        match byte_char {
            'i' => {
                raw = self.start_tag_in(vec!["iframe".to_string()]);
            },
            'n' => {
                raw = self.start_tag_in(vec!["noembed".to_string(), "noframes".to_string(), "noscript".to_string()]);
            },
            'p' => {
                raw = self.start_tag_in(vec!["plaintext".to_string()]);
            },
            's' => {
                raw = self.start_tag_in(vec!["script".to_string(), "style".to_string()]);
            },
            't' => {
                raw = self.start_tag_in(vec!["textarea".to_string(), "title".to_string()]);
            },
            'x' => {
                raw = self.start_tag_in(vec!["xmp".to_string()]);
            },
            _ => {}
        }

        if raw {
            self.raw_tag = str::from_utf8(&self.buffer[self.data.start..self.data.end]).expect("").to_string().clone().to_lowercase();
        }

        if self.err.is_none() && self.buffer[self.raw.end - 2] == '/' as u8 {
            return SelfClosingTagToken;
        }

        return StartTagToken;
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
                },
                '/' | '>' => {
                    self.raw.end -= 1;
                    self.data.end = self.raw.end;

                    return;
                },
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
                },
                '=' | '>' => {
                    self.raw.end -= 1;
                    self.pending_attribute[0].end = self.raw.end;

                    return;
                },
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

                return;
            },
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
            },
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
                        },
                        '>' => {
                            self.raw.end -= 1;
                            self.pending_attribute[1].end = self.raw.end;

                            return;
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}

// convertNewlines converts "\r" and "\r\n" in s to "\n".
// The conversion happens in place, but the resulting slice may be shorter.
fn convert_next_lines(s: String) -> String {
    let bytes = s.as_bytes();
    let mut new_bytes = Vec::new();

    for i in 0..bytes.len() {
        let c = bytes[i];

        if c != '\r' as u8 {
            new_bytes.push(c);

            continue;
        }

        let mut src = i + 1;

        if src >= bytes.len() || bytes[src] as char != '\n' {
            new_bytes.push('\n' as u8);
        }

        let dst = i;

        while src < bytes.len() {
            if bytes[src] == '\r' as u8 {
                if src + 1 < bytes.len() && bytes[src + 1] == '\n' as u8 {
                    src += 1;
                }

                new_bytes.push('\n' as u8);
            } else {
                new_bytes.push(bytes[src]);
            }
        }
    }

    return str::from_utf8(new_bytes.as_slice()).expect("Cannot create string").to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TokenTest {
        desc: String,
        html: String,
        golden: String,
    }

    #[test]
    pub fn test_tokenizer() {
        let token_tests = vec![
            TokenTest { desc: "empty".to_string(), html: "".to_string(), golden: "".to_string() },
            TokenTest { desc: "text".to_string(), html: "foo  bar".to_string(), golden: "foo  bar".to_string() },
            TokenTest { desc: "entity".to_string(), html: "one &lt; two".to_string(), golden: "one &lt; two".to_string() },
            TokenTest { desc: "tags".to_string(), html: "<a>b<c/>d</e>".to_string(), golden: "<a>$b$<c/>$d$</e>".to_string() },
            TokenTest { desc: "not a tag #0".to_string(), html: "<".to_string(), golden: "<".to_string() },
            TokenTest { desc: "not a tag #1".to_string(), html: "</".to_string(), golden: "</".to_string() },
            TokenTest { desc: "not a tag #2".to_string(), html: "</>".to_string(), golden: "<!---->".to_string() },
            TokenTest { desc: "not a tag #3".to_string(), html: "a</>b".to_string(), golden: "a$<!---->$b".to_string() },
            TokenTest { desc: "not a tag #4".to_string(), html: "</ >".to_string(), golden: "<!-- -->".to_string() },
            TokenTest { desc: "not a tag #5".to_string(), html: "</.".to_string(), golden: "<!--.-->".to_string() },
            TokenTest { desc: "not a tag #6".to_string(), html: "</.>".to_string(), golden: "<!--.-->".to_string() },
            TokenTest { desc: "not a tag #7".to_string(), html: "a < b".to_string(), golden: "a < b".to_string() },
            TokenTest { desc: "not a tag #8".to_string(), html: "<.>".to_string(), golden: "<.>".to_string() },
            TokenTest { desc: "not a tag #9".to_string(), html: "a<<<b>>>c".to_string(), golden: "a<<$<b>$>>c".to_string() },
            TokenTest { desc: "not a tag #10".to_string(), html: "i x<0 and y < 0 then x*y>0".to_string(), golden: "i x<0 and y < 0 then x*y>0".to_string() },
            TokenTest { desc: "not a tag #11".to_string(), html: "<<p>".to_string(), golden: "<$<p>".to_string() },
            TokenTest { desc: "tag name eof #0".to_string(), html: "<a".to_string(), golden: "".to_string() },
            TokenTest { desc: "tag name eof #1".to_string(), html: "<a ".to_string(), golden: "".to_string() },
            TokenTest { desc: "tag name eof #2".to_string(), html: "a<b ".to_string(), golden: "a".to_string() },
            TokenTest { desc: "tag name eof #3".to_string(), html: "<a><b ".to_string(), golden: "<a>".to_string() },
            TokenTest { desc: "tag name eof #4".to_string(), html: "<a x ".to_string(), golden: "".to_string() },
            TokenTest { desc: "malformed tag #0".to_string(), html: "<p</p>".to_string(), golden: "<p< p=\"\">".to_string() },
            TokenTest { desc: "malformed tag #1".to_string(), html: "<p </p>".to_string(), golden: "<p <=\"\" p=\"\">".to_string() },
            TokenTest { desc: "malformed tag #2".to_string(), html: "<p id".to_string(), golden: "".to_string() },
            TokenTest { desc: "malformed tag #3".to_string(), html: "<p id=".to_string(), golden: "".to_string() },
            TokenTest { desc: "malformed tag #4".to_string(), html: "<p id=>".to_string(), golden: "<p id=\"\">".to_string() },
            TokenTest { desc: "malformed tag #5".to_string(), html: "<p id=0".to_string(), golden: "".to_string() },
            TokenTest { desc: "malformed tag #6".to_string(), html: "<p id=0</p>".to_string(), golden: "<p id=\"0</p\">".to_string() },
            TokenTest { desc: "malformed tag #7".to_string(), html: "<p id=\"0</p>".to_string(), golden: "".to_string() },
            TokenTest { desc: "malformed tag #8".to_string(), html: "<p id=\"0\"</p>".to_string(), golden: "<p id=\"0\" <=\"\" p=\"\">".to_string() },
            TokenTest { desc: "malformed tag #9".to_string(), html: "<p></p id".to_string(), golden: "<p>".to_string() },
            TokenTest { desc: "basic raw text".to_string(), html: "<script><a></b></script>".to_string(), golden: "<script>$<a></b>$</script>".to_string() },
            TokenTest { desc: "unfinished script end tag".to_string(), html: "<SCRIPT>a</SCR".to_string(), golden: "<script>$a</SCR".to_string() },
            TokenTest { desc: "broken script end tag".to_string(), html: "<SCRIPT>a</SCR ipt>".to_string(), golden: "<script>$a</SCR ipt>".to_string() },
            TokenTest { desc: "EOF in script end tag".to_string(), html: "<SCRIPT>a</SCRipt".to_string(), golden: "<script>$a</SCRipt".to_string() },
            TokenTest { desc: "scriptx end tag".to_string(), html: "<SCRIPT>a</SCRiptx".to_string(), golden: "<script>$a</SCRiptx".to_string() },
            TokenTest { desc: "' ' completes script end tag".to_string(), html: "<SCRIPT>a</SCRipt ".to_string(), golden: "<script>$a".to_string() },
            TokenTest { desc: "'>' completes script end tag".to_string(), html: "<SCRIPT>a</SCRipt>".to_string(), golden: "<script>$a$</script>".to_string() },
            TokenTest { desc: "'>' completes script end tag".to_string(), html: "<SCRIPT>a</SCRipt>".to_string(), golden: "<script>$a$</script>".to_string() },
            TokenTest { desc: "nested script tag".to_string(), html: "<SCRIPT>a</SCRipt<script>".to_string(), golden: "<script>$a</SCRipt<script>".to_string() },
            TokenTest { desc: "script end tag after unfinished".to_string(), html: "<SCRIPT>a</SCRipt</script>".to_string(), golden: "<script>$a</SCRipt$</script>".to_string() },
            TokenTest { desc: "script/style mismatched tags".to_string(), html: "<script>a</style>".to_string(), golden: "<script>$a</style>".to_string() },
            TokenTest { desc: "style element with entity".to_string(), html: "<style>&apos;".to_string(), golden: "<style>$&apos;".to_string() },
            TokenTest { desc: "textarea with tag".to_string(), html: "<textarea><div></textarea>".to_string(), golden: "<textarea>$<div>$</textarea>".to_string() },
            TokenTest { desc: "title with tag and entity".to_string(), html: "<title><b>K&amp;R C</b></title>".to_string(), golden: "<title>$<b>K&amp;R C</b>$</title>".to_string() },
            TokenTest { desc: "Proper DOCTYPE".to_string(), html: "<!DOCTYPE html>".to_string(), golden: "<!DOCTYPE html>".to_string() },
            TokenTest { desc: "DOCTYPE with no space".to_string(), html: "<!doctypehtml>".to_string(), golden: "<!DOCTYPE html>".to_string() },
            TokenTest { desc: "DOCTYPE with two space".to_string(), html: "<!doctype  html>".to_string(), golden: "<!DOCTYPE html>".to_string() },
            TokenTest { desc: "looks like DOCTYPE but isn't".to_string(), html: "<!DOCUMENT html>".to_string(), golden: "<!--DOCUMENT html-->".to_string() },
            TokenTest { desc: "DOCTYPE at EOF".to_string(), html: "<!DOCTYPE".to_string(), golden: "<!DOCTYPE >".to_string() },
            TokenTest { desc: "XML Processing instruction".to_string(), html: "<?xml?>".to_string(), golden: "<!--?xml?-->".to_string() },
            TokenTest { desc: "comment #0".to_string(), html: "abc<b><!-- skipme --></b>def".to_string(), golden: "abc$<b>$<!-- skipme -->$</b>$def".to_string() },
            TokenTest { desc: "comment #1".to_string(), html: "a<!-->z".to_string(), golden: "a$<!---->$z".to_string() },
            TokenTest { desc: "comment #2".to_string(), html: "a<!--->z".to_string(), golden: "a$<!---->$z".to_string() },
            TokenTest { desc: "comment #3".to_string(), html: "a<!--x>-->z".to_string(), golden: "a$<!--x>-->$z".to_string() },
            TokenTest { desc: "comment #4".to_string(), html: "a<!--x->-->z".to_string(), golden: "a$<!--x->-->$z".to_string() },
            TokenTest { desc: "comment #5".to_string(), html: "a<!>z".to_string(), golden: "a$<!---->$z".to_string() },
            TokenTest { desc: "comment #6".to_string(), html: "a<!->z".to_string(), golden: "a$<!----->$z".to_string() },
            TokenTest { desc: "comment #7".to_string(), html: "a<!---<>z".to_string(), golden: "a$<!---<>z-->".to_string() },
            TokenTest { desc: "comment #8".to_string(), html: "a<!--z".to_string(), golden: "a$<!--z-->".to_string() },
            TokenTest { desc: "comment #9".to_string(), html: "a<!--z-".to_string(), golden: "a$<!--z-->".to_string() },
            TokenTest { desc: "comment #10".to_string(), html: "a<!--z--".to_string(), golden: "a$<!--z-->".to_string() },
            TokenTest { desc: "comment #11".to_string(), html: "a<!--z---".to_string(), golden: "a$<!--z--->".to_string() },
            TokenTest { desc: "comment #12".to_string(), html: "a<!--z----".to_string(), golden: "a$<!--z---->".to_string() },
            TokenTest { desc: "comment #13".to_string(), html: "a<!--x--!>z".to_string(), golden: "a$<!--x-->$z".to_string() },
            TokenTest { desc: "backslash".to_string(), html: "<p id=\"a\\\"b\">".to_string(), golden: "<p id=\"a\\\" b\"=\"\">".to_string() },
            TokenTest { desc: "tricky".to_string(), html: "<p \t\n iD=\"a&quot;B\"  foo=\"bar\"><EM>te&lt;&amp;;xt</em></p>".to_string(), golden: "<p id=\"a&quot;B\" foo=\"bar\">$<em>$te&lt;&amp;;xt$</em>$</p>".to_string() },
            TokenTest { desc: "noSuchEntity".to_string(), html: "<a b=\"c&noSuchEntity;d\">&lt;&alsoDoesntExist;&".to_string(), golden: "<a b=\"c&noSuchEntity;d\">$&lt;&alsoDoesntExist;&".to_string() },
            TokenTest { desc: "entity without semicolon".to_string(), html: "&notit;&notin;<a b=\"q=z&amp=5&notice=hello&not;=world\">".to_string(), golden: "&notit;&notin;$<a b=\"q=z&amp=5&notice=hello&not;=world\">".to_string() },
            TokenTest { desc: "Empty attribute".to_string(), html: "<input disabled FOO>".to_string(), golden: "<input disabled=\"\" foo=\"\">".to_string() },
            TokenTest { desc: "Empty attribute, whitespace".to_string(), html: "<input disabled FOO >".to_string(), golden: "<input disabled=\"\" foo=\"\">".to_string() },
            TokenTest { desc: "unqoted attribute".to_string(), html: "<input value=yes FOO=BAR>".to_string(), golden: "<input value=\"yes\" foo=\"BAR\">".to_string() },
            TokenTest { desc: "unqoted attribute spaces".to_string(), html: "<input value = yes FOO = BAR>".to_string(), golden: "<input value=\"yes\" foo=\"BAR\">".to_string() },
            TokenTest { desc: "unqoted attribute, trailing space".to_string(), html: "<input value=yes FOO=BAR >".to_string(), golden: "<input value=\"yes\" foo=\"BAR\">".to_string() },
            TokenTest { desc: "Single-quoted attribute value".to_string(), html: "<input value='yes' FOO='BAR'>".to_string(), golden: "<input value=\"yes\" foo=\"BAR\">".to_string() },
            TokenTest { desc: "Single-quoted attribute value, trailing space".to_string(), html: "<input value='yes' FOO='BAR' >".to_string(), golden: "<input value=\"yes\" foo=\"BAR\">".to_string() },
            TokenTest { desc: "Double-quoted attribute value".to_string(), html: "<input value=\"I'm an attribute\" FOO=\"BAR\">".to_string(), golden: "<input value=\"I'm an attribute\" foo=\"BAR\">".to_string() },
            TokenTest { desc: "Attribute name characters".to_string(), html: "<meta http-equiv=\"content-type\">".to_string(), golden: "<meta http-equiv=\"content-type\">".to_string() },
            TokenTest { desc: "Mixed attributes".to_string(), html: "a<P V=\"0 1\" w='2' X=3 y>z".to_string(), golden: "a$<p v=\"0 1\" w=\"2\" x=\"3\" y=\"\">$z".to_string() },
            TokenTest { desc: "Attributes with a solitary single quote".to_string(), html: "<p id=can't><p id=won't>".to_string(), golden: "<p id=\"can't\">$<p id=\"won't\">".to_string() },
        ];

        'test: for tt in token_tests {
            let mut html = tt.html.clone();
            let mut reader = &mut html.as_bytes() as &mut std::io::Read;
            let mut tokenizer = Tokenizer::new(reader);

            if !tt.golden.is_empty() {
                let splits = tt.golden.split("$");

                for split in splits {
                    let token_type = tokenizer.next();

                    if token_type == ErrorToken {
                        panic!("{:?} token", token_type);

                        continue 'test;
                    }

                    let actual_token = tokenizer.token();

                    println!("{:?}", actual_token);

                    if actual_token.to_string() != split {
                        panic!("received: '{}', expected: '{}'", actual_token.to_string(), split);

                        continue 'test;
                    }
                }
            }

            tokenizer.next();

            if tokenizer.err().is_none() {
                panic!("Error expected");
            }
        }
    }
}
