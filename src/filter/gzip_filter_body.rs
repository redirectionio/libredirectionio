use crate::filter::error::Result;
use flate2::write::{GzDecoder, GzEncoder};
use std::io::Write;

#[derive(Debug)]
pub struct GzDecodeFilterBody {
    decoder: GzDecoder<Vec<u8>>,
}

#[derive(Debug)]
pub struct GzEncodeFilterBody {
    encoder: GzEncoder<Vec<u8>>,
}

impl GzDecodeFilterBody {
    pub fn new() -> Self {
        Self {
            decoder: GzDecoder::new(Vec::new()),
        }
    }

    pub fn filter(&mut self, data: Vec<u8>) -> Result<Vec<u8>> {
        self.decoder.write_all(data.as_slice())?;
        self.decoder.flush()?;

        if self.decoder.get_ref().is_empty() {
            return Ok(Vec::new());
        }

        let mut buffer = Vec::new();
        std::mem::swap(&mut buffer, &mut self.decoder.get_mut());

        Ok(buffer)
    }

    pub fn end(&mut self) -> Result<Vec<u8>> {
        let mut decoder = GzDecoder::new(Vec::new());
        std::mem::swap(&mut self.decoder, &mut decoder);

        decoder.try_finish()?;
        Ok(decoder.finish()?)
    }
}

impl GzEncodeFilterBody {
    pub fn new() -> Self {
        Self {
            encoder: GzEncoder::new(Vec::new(), flate2::Compression::default()),
        }
    }

    pub fn filter(&mut self, data: Vec<u8>) -> Result<Vec<u8>> {
        self.encoder.write_all(data.as_slice())?;
        self.encoder.flush()?;

        if self.encoder.get_ref().is_empty() {
            return Ok(Vec::new());
        }

        let mut buffer = Vec::new();
        std::mem::swap(&mut buffer, &mut self.encoder.get_mut());

        Ok(buffer)
    }

    pub fn end(&mut self) -> Result<Vec<u8>> {
        let mut encoder = GzEncoder::new(Vec::new(), flate2::Compression::default());
        std::mem::swap(&mut self.encoder, &mut encoder);

        encoder.try_finish()?;
        Ok(encoder.finish()?)
    }
}
