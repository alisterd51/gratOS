const BUFFER_SIZE: usize = 500;

pub struct FifoBuffer {
    begin: usize,
    end: usize,
    data: [u64; BUFFER_SIZE],
}

impl FifoBuffer {
    pub const fn new() -> Self {
        Self {
            begin: 0,
            end: 0,
            data: [0u64; BUFFER_SIZE],
        }
    }

    pub const fn push(&mut self, val: u64) -> Result<(), ()> {
        if (self.end + 1) % BUFFER_SIZE == self.begin {
            return Err(());
        }
        self.data[self.end] = val;
        self.end = (self.end + 1) % BUFFER_SIZE;
        Ok(())
    }

    pub const fn pop(&mut self) -> Option<u64> {
        if self.end == self.begin {
            return None;
        }
        let val = self.data[self.begin];
        self.begin = (self.begin + 1) % BUFFER_SIZE;
        Some(val)
    }
}
