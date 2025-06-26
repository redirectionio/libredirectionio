use std::{
    fmt::{Debug, Formatter},
    io::Write,
};

use brotli::DecompressorWriter;
use flate2::write::{GzDecoder, ZlibDecoder};

use crate::filter::{encoding::SupportedEncoding, error::Result};

pub enum DecodeFilterBody {
    Gzip(GzDecoder<Vec<u8>>),
    Brotli(Box<DecompressorWriter<Vec<u8>>>),
    Deflate(ZlibDecoder<Vec<u8>>),
}

impl Debug for DecodeFilterBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DecodeFilterBody").finish()
    }
}

impl DecodeFilterBody {
    pub fn new(encoding: SupportedEncoding) -> Self {
        match encoding {
            SupportedEncoding::Brotli => Self::Brotli(Box::new(DecompressorWriter::new(Vec::new(), 4096))),
            SupportedEncoding::Gzip => Self::Gzip(GzDecoder::new(Vec::new())),
            SupportedEncoding::Deflate => Self::Deflate(ZlibDecoder::new(Vec::new())),
        }
    }

    pub fn filter(&mut self, data: Vec<u8>) -> Result<Vec<u8>> {
        match self {
            Self::Deflate(decoder) => {
                decoder.write_all(data.as_slice())?;
                decoder.flush()?;

                if decoder.get_ref().is_empty() {
                    return Ok(Vec::new());
                }

                let mut buffer = Vec::new();
                std::mem::swap(&mut buffer, decoder.get_mut());

                Ok(buffer)
            }
            Self::Gzip(decoder) => {
                decoder.write_all(data.as_slice())?;
                decoder.flush()?;

                if decoder.get_ref().is_empty() {
                    return Ok(Vec::new());
                }

                let mut buffer = Vec::new();
                std::mem::swap(&mut buffer, decoder.get_mut());

                Ok(buffer)
            }
            Self::Brotli(decoder) => {
                decoder.write_all(data.as_slice())?;
                decoder.flush()?;

                if decoder.get_ref().is_empty() {
                    return Ok(Vec::new());
                }

                let mut buffer = Vec::new();
                std::mem::swap(&mut buffer, decoder.get_mut());

                Ok(buffer)
            }
        }
    }

    pub fn end(self) -> Result<Vec<u8>> {
        match self {
            Self::Deflate(mut d) => {
                d.try_finish()?;
                Ok(d.finish()?)
            }
            Self::Gzip(mut d) => {
                d.try_finish()?;
                Ok(d.finish()?)
            }
            Self::Brotli(d) => match d.into_inner() {
                Ok(buffer) => Ok(buffer),
                Err(buffer) => Ok(buffer),
            },
        }
    }
}
