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

    pub fn filter(&mut self, data: Vec<u8>) -> Vec<u8> {
        let mut decoded = vec![0u8; BUFFER_FRAME_SIZE];

        match self.decoder.write_all(data.as_slice()) {
            Ok(()) => {}
            Err(err) => {
                log::error!("Error while decoding gzip: {}", err);

                return data;
            }
        }

        self.decoder.read(&mut decoded).expect("Error while decoding gzip");

        decoded

        // match self.decoder.read(&mut decoded) {
        //     Ok(size) => decoded[..size].to_vec(),
        //     Err(err) => {
        //         panic!("Error while decoding gzip: {}", err);
        //     }
        // }
    }

    pub fn end(&mut self) -> Vec<u8> {
        let mut decoder = GzDecoder::new(Cursor::new(Vec::new()));
        std::mem::swap(&mut self.decoder, &mut decoder);

        match decoder.finish() {
            Ok(data) => data.into_inner(),
            Err(err) => {
                log::error!("Error while encoding gzip: {}", err);

                Vec::new()
            }
        }
    }
}

impl GzEncodeFilterBody {
    pub fn new() -> Self {
        Self {
            encoder: GzEncoder::new(Cursor::new(Vec::new()), flate2::Compression::default()),
        }
    }

    pub fn filter(&mut self, data: Vec<u8>) -> Vec<u8> {
        let mut encoded = vec![0u8; BUFFER_FRAME_SIZE];

        match self.encoder.write_all(data.as_slice()) {
            Ok(()) => {}
            Err(err) => {
                panic!("Error while encoding gzip: {}", err);

                return data;
            }
        }

        match self.encoder.read(&mut encoded) {
            Ok(size) => encoded[..size].to_vec(),
            Err(err) => {
                log::error!("Error while encoding gzip: {}", err);

                return data;
            }
        }
    }

    pub fn end(&mut self) -> Vec<u8> {
        let mut encoder = GzEncoder::new(Cursor::new(Vec::new()), flate2::Compression::default());
        std::mem::swap(&mut self.encoder, &mut encoder);

        match encoder.finish() {
            Ok(data) => data.into_inner(),
            Err(err) => {
                log::error!("Error while encoding gzip: {}", err);

                Vec::new()
            }
        }
    }
}
