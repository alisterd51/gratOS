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

    fn find_free_region(&self, count: usize) -> Option<usize> {
        let mut free_count = 0;
        let mut start_idx = 0;

        for array_index in 0..BITMAP_SIZE {
            let word = self.bitmap[array_index];

            if word == usize::MAX {
                free_count = 0;
                continue;
            }
            for bit_index in 0..(usize::BITS as usize) {
                if (word & (1 << bit_index)) == 0 {
                    if free_count == 0 {
                        start_idx = array_index * usize::BITS as usize + bit_index;
                    }
                    free_count += 1;
                    if free_count == count {
                        return Some(start_idx);
                    }
                } else {
                    free_count = 0;
                }
            }
        }
        None
    }

    fn mark_range_as_used(&mut self, start: usize, count: usize) {
        let mut remaining = count;
        let mut current_bit = start;
        let bits_per_word = usize::BITS as usize;

        while remaining > 0 {
            let idx = current_bit / bits_per_word;
            let bit = current_bit % bits_per_word;
            let bits_to_set = remaining.min(bits_per_word - bit);
            let mask = if bits_to_set == bits_per_word {
                usize::MAX
            } else {
                ((1usize << bits_to_set) - 1) << bit
            };

            self.bitmap[idx] |= mask;
            remaining -= bits_to_set;
            current_bit += bits_to_set;
        }
    }

    pub fn allocate_contiguous_frames(&mut self, count: usize) -> Option<PhysFrame> {
        if count == 0 {
            return None;
        }
        if count == 1 {
            return self.allocate_frame();
        }

        let start = self.find_free_region(count)?;

        self.mark_range_as_used(start, count);

        if start == self.last_free_index {
            self.last_free_index = start + count;
        }

        Some(PhysFrame { number: start })
    }

    pub fn count_free_frames(&self) -> usize {
        let mut free = 0;
        for word in &self.bitmap {
            free += word.count_zeros() as usize;
        }
        free
    }
}
