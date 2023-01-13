use crate::filter::encode::SupportedEncoding;
use crate::filter::error::Result;
use brotli::CompressorWriter;
use flate2::write::{GzEncoder, ZlibEncoder};
use std::fmt::{Debug, Formatter};
use std::io::Write;

pub enum EncodeFilterBody {
    Gzip(GzEncoder<Vec<u8>>),
    Brotli(CompressorWriter<Vec<u8>>),
    Deflate(ZlibEncoder<Vec<u8>>),
}

impl Debug for EncodeFilterBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncodeFilterBody").finish()
    }
}

impl EncodeFilterBody {
    pub fn new(encoding: SupportedEncoding) -> Self {
        match encoding {
            SupportedEncoding::Brotli => Self::Brotli(CompressorWriter::new(Vec::new(), 4096, 11, 22)),
            SupportedEncoding::Gzip => Self::Gzip(GzEncoder::new(Vec::new(), flate2::Compression::default())),
            SupportedEncoding::Deflate => Self::Deflate(ZlibEncoder::new(Vec::new(), flate2::Compression::default())),
        }
    }

    pub fn filter(&mut self, data: Vec<u8>) -> Result<Vec<u8>> {
        match self {
            Self::Deflate(encoder) => {
                encoder.write_all(data.as_slice())?;
                encoder.flush()?;

                if encoder.get_ref().is_empty() {
                    return Ok(Vec::new());
                }

                let mut buffer = Vec::new();
                std::mem::swap(&mut buffer, &mut encoder.get_mut());

                Ok(buffer)
            }
            Self::Gzip(encoder) => {
                encoder.write_all(data.as_slice())?;
                encoder.flush()?;

                if encoder.get_ref().is_empty() {
                    return Ok(Vec::new());
                }

                let mut buffer = Vec::new();
                std::mem::swap(&mut buffer, &mut encoder.get_mut());

                Ok(buffer)
            }
            Self::Brotli(encoder) => {
                encoder.write_all(data.as_slice())?;
                encoder.flush()?;

                if encoder.get_ref().is_empty() {
                    return Ok(Vec::new());
                }

                let mut buffer = Vec::new();
                std::mem::swap(&mut buffer, &mut encoder.get_mut());

                Ok(buffer)
            }
        }
    }

    pub fn end(&mut self) -> Result<Vec<u8>> {
        match self {
            Self::Deflate(d) => {
                let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
                std::mem::swap(d, &mut encoder);

                encoder.try_finish()?;
                Ok(encoder.finish()?)
            }
            Self::Gzip(d) => {
                let mut encoder = GzEncoder::new(Vec::new(), flate2::Compression::default());
                std::mem::swap(d, &mut encoder);

                encoder.try_finish()?;
                Ok(encoder.finish()?)
            }
            Self::Brotli(d) => {
                let mut compressor = CompressorWriter::new(Vec::new(), 4096, 11, 22);
                std::mem::swap(&mut compressor, d);

                Ok(compressor.into_inner())
            }
        }
    }
}
