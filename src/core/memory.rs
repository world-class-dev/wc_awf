/// Zero-Allocation Linear Memory Buffer (Sandboxed Arena)
pub struct LinearMemoryBuffer {
    buffer: Vec<u8>,
    offset: usize,
}

impl LinearMemoryBuffer {
    pub fn new(capacity: usize) -> Self {
        LinearMemoryBuffer {
            buffer: vec![0u8; capacity],
            offset: 0,
        }
    }

    pub fn allocate(&mut self, size: usize) -> Option<&mut [u8]> {
        if self.offset + size > self.buffer.len() {
            return None;
        }
        let start = self.offset;
        self.offset += size;
        Some(&mut self.buffer[start..self.offset])
    }

    pub fn reset(&mut self) {
        self.offset = 0;
    }

    pub fn available_bytes(&self) -> usize {
        self.buffer.len() - self.offset
    }
}
