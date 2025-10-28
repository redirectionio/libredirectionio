#[derive(Debug, Default)]
pub struct BufferFilterBody {
    buffer: Vec<u8>,
}

impl BufferFilterBody {
    pub fn filter(&mut self, input: Vec<u8>) -> Vec<u8> {
        self.buffer.extend_from_slice(&input);
        Vec::new()
    }

    pub fn end(self) -> Vec<u8> {
        self.buffer
    }
}
