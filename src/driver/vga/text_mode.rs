use super::{ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH};
use crate::io::outb;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new() -> Self {
        Writer {
            buffer: unsafe { &mut *(0xB8000 as *mut Buffer) },
        }
    }

    pub fn set_char(&mut self, c: ScreenChar, col: usize, row: usize) {
        self.buffer.chars[row][col] = c;
    }

    pub fn set_line(&mut self, line: [ScreenChar; BUFFER_WIDTH], row: usize) {
        self.buffer.chars[row] = line;
    }

    pub fn set_screen(&mut self, screen: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]) {
        self.buffer.chars = screen;
    }
}

pub fn set_cursor(col: usize, row: usize) {
    let pos = (row * BUFFER_WIDTH + col) as u16;

    unsafe {
        outb(0xF, 0x3D4);
        outb((pos & 0xFF) as u8, 0x3D5);
        outb(0xE, 0x3D4);
        outb(((pos >> 8) & 0xFF) as u8, 0x3D5);
    }
}
