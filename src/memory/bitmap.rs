use crate::memory::PhysFrame;

const MAX_FRAMES: usize = 1024 * 1024;
const BITMAP_SIZE: usize = MAX_FRAMES / (usize::BITS as usize);

pub struct BitmapAllocator {
    bitmap: [usize; BITMAP_SIZE],
    last_free_index: usize,
}

#[allow(clippy::large_stack_arrays)]
impl BitmapAllocator {
    pub const fn new() -> Self {
        Self {
            bitmap: [usize::MAX; BITMAP_SIZE],
            last_free_index: 0,
        }
    }

    const fn set_bit(&mut self, frame: PhysFrame, value: bool) {
        let frame_number = frame.number;

        if frame_number < MAX_FRAMES {
            let array_index = frame_number / (usize::BITS as usize);
            let bit_index = frame_number % (usize::BITS as usize);
            let bit_mask = 1 << bit_index;

            if value {
                self.bitmap[array_index] |= bit_mask;
            } else {
                self.bitmap[array_index] &= !bit_mask;
            }
        }
    }

    pub const fn free_frame(&mut self, frame: PhysFrame) {
        self.set_bit(frame, false);
        if self.last_free_index > frame.number {
            self.last_free_index = frame.number;
        }
    }

    pub const fn lock_frame(&mut self, frame: PhysFrame) {
        self.set_bit(frame, true);
    }

    pub fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let start_array_index = self.last_free_index / (usize::BITS as usize);

        for array_index in start_array_index..BITMAP_SIZE {
            let mut word = self.bitmap[array_index];

            if array_index == start_array_index {
                let bit_offset = self.last_free_index % (usize::BITS as usize);
                let mask = (1usize << bit_offset) - 1;

                word |= mask;
            }
            if word != usize::MAX {
                let bit_index = word.trailing_ones() as usize;
                let number = array_index * (usize::BITS as usize) + bit_index;

                if number < MAX_FRAMES {
                    self.bitmap[array_index] |= 1 << bit_index;
                    self.last_free_index = number + 1;

                    return Some(PhysFrame { number });
                }
                return None;
            }
        }
        None
    }
}
