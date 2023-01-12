mod decode;
mod encode;

pub enum SupportedEncoding {
    Brotli,
    Gzip,
    Deflate,
}
