mod decode;
mod encode;

#[derive(Clone)]
pub enum SupportedEncoding {
    Brotli,
    Gzip,
    Deflate,
}

impl SupportedEncoding {
    pub fn new_hash_set() -> HashSet<String> {
        let mut set = HashSet::new();
        set.insert("br".to_string());
        set.insert("gzip".to_string());
        set.insert("deflate".to_string());
        set
    }
}

use std::collections::HashSet;
pub use decode::DecodeFilterBody;
pub use encode::EncodeFilterBody;

pub fn get_encoding_filters(encoding: &str) -> Option<(DecodeFilterBody, EncodeFilterBody)> {
    let supported_encoding = match encoding {
        "br" => SupportedEncoding::Brotli,
        "gzip" => SupportedEncoding::Gzip,
        "deflate" => SupportedEncoding::Deflate,
        _ => return None,
    };

    Some((
        DecodeFilterBody::new(supported_encoding.clone()),
        EncodeFilterBody::new(supported_encoding),
    ))
}
