use crate::filter::error::Result;
use brotli::{CompressorWriter, DecompressorWriter};
use std::fmt::{Debug, Formatter};
use std::io::Write;

pub struct BrotliDecodeFilterBody {
    decompressor: DecompressorWriter<Vec<u8>>,
}

pub struct BrotliEncodeFilterBody {
    compressor: CompressorWriter<Vec<u8>>,
}

impl Debug for BrotliDecodeFilterBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BrotliDecodeFilterBody").finish()
    }
}

impl Debug for BrotliEncodeFilterBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BrotliEncodeFilterBody").finish()
    }
}

impl BrotliDecodeFilterBody {
    pub fn new() -> Self {
        Self {
            decompressor: DecompressorWriter::new(Vec::new(), 4096),
        }
    }

    pub fn filter(&mut self, data: Vec<u8>) -> Result<Vec<u8>> {
        self.decompressor.write_all(data.as_slice())?;
        self.decompressor.flush()?;

        if self.decompressor.get_ref().is_empty() {
            return Ok(Vec::new());
        }

        let mut buffer = Vec::new();
        std::mem::swap(&mut buffer, &mut self.decompressor.get_mut());

        Ok(buffer)
    }

    pub fn end(&mut self) -> Result<Vec<u8>> {
        let mut decompressor = DecompressorWriter::new(Vec::new(), 4096);
        std::mem::swap(&mut decompressor, &mut self.decompressor);

        match decompressor.into_inner() {
            Ok(buffer) => Ok(buffer),
            Err(buffer) => Ok(buffer),
        }
    }
}

impl BrotliEncodeFilterBody {
    pub fn new() -> Self {
        Self {
            compressor: CompressorWriter::new(Vec::new(), 4096, 11, 22),
        }
    }

    pub fn filter(&mut self, data: Vec<u8>) -> Result<Vec<u8>> {
        self.compressor.write_all(&data)?;
        self.compressor.flush()?;

        if self.compressor.get_ref().is_empty() {
            return Ok(Vec::new());
        }

        let mut buffer = Vec::new();
        std::mem::swap(&mut buffer, &mut self.compressor.get_mut());

        Ok(buffer)
    }

    pub fn end(&mut self) -> Result<Vec<u8>> {
        let mut compressor = CompressorWriter::new(Vec::new(), 4096, 11, 22);
        std::mem::swap(&mut compressor, &mut self.compressor);

        Ok(compressor.into_inner())
    }
}
