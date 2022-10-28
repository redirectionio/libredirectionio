use crate::filter::error::Result;
use flate2::write::{GzDecoder, GzEncoder};
use std::io::{Cursor, Read, Write};

pub struct GzDecodeFilterBody {
    decoder: GzDecoder<Cursor<Vec<u8>>>,
}

pub struct GzEncodeFilterBody {
    encoder: GzEncoder<Cursor<Vec<u8>>>,
}

static BUFFER_FRAME_SIZE: usize = 4096;

impl GzDecodeFilterBody {
    pub fn new() -> Self {
        Self {
            decoder: GzDecoder::new(Cursor::new(Vec::new())),
        }
    }

    pub fn filter(&mut self, data: Vec<u8>) -> Result<Vec<u8>> {
        let mut decoded = vec![0u8; BUFFER_FRAME_SIZE];

        self.decoder.write_all(data.as_slice())?;
        let readed = self.decoder.read(&mut decoded)?;

        Ok(decoded[..readed].to_vec())
    }

    pub fn end(&mut self) -> Result<Vec<u8>> {
        let mut decoder = GzDecoder::new(Cursor::new(Vec::new()));
        std::mem::swap(&mut self.decoder, &mut decoder);

        Ok(decoder.finish()?.into_inner())
    }
}

impl GzEncodeFilterBody {
    pub fn new() -> Self {
        Self {
            encoder: GzEncoder::new(Cursor::new(Vec::new()), flate2::Compression::default()),
        }
    }

    pub fn filter(&mut self, data: Vec<u8>) -> Result<Vec<u8>> {
        let mut encoded = vec![0u8; BUFFER_FRAME_SIZE];

        self.encoder.write_all(data.as_slice())?;
        let readed = self.encoder.read(&mut encoded)?;

        Ok(encoded[..readed].to_vec())
    }

    pub fn end(&mut self) -> Result<Vec<u8>> {
        let mut encoder = GzEncoder::new(Cursor::new(Vec::new()), flate2::Compression::default());
        std::mem::swap(&mut self.encoder, &mut encoder);

        Ok(encoder.finish()?.into_inner())
    }
}
