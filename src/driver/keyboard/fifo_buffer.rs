const BUFFER_SIZE: usize = 500;

pub struct FifoBuffer {
    begin: usize,
    end: usize,
    chars: [u16; BUFFER_SIZE],
}

impl FifoBuffer {
    pub const fn new() -> Self {
        Self {
            begin: 0,
            end: 0,
            chars: [0u16; BUFFER_SIZE],
        }
    }

    pub const fn push(&mut self, val: u16) -> Result<(), ()> {
        if (self.end + 1) % BUFFER_SIZE == self.begin {
            return Err(());
        }
        self.chars[self.end] = val;
        self.end = (self.end + 1) % BUFFER_SIZE;
        Ok(())
    }

    pub const fn pop(&mut self) -> Option<u16> {
        if self.end == self.begin {
            return None;
        }
        let val = self.chars[self.begin];
        self.begin = (self.begin + 1) % BUFFER_SIZE;
        Some(val)
    }
}
