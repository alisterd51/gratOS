use super::{BUFFER_WIDTH, Screen, ScreenChar, ScreenCharLine};
use crate::io::outb;
use core::ptr::write_volatile;

#[repr(transparent)]
struct Buffer {
    chars: Screen,
}

unsafe impl Send for Writer {}

pub struct Writer {
    buffer: *mut Buffer,
}

impl Writer {
    pub const fn new() -> Self {
        Self {
            buffer: 0xB8000 as *mut Buffer,
        }
    }

    #[allow(clippy::volatile_composites)]
    pub fn set_char(&mut self, c: &ScreenChar, col: usize, row: usize) {
        unsafe {
            let dst = &raw mut (*self.buffer).chars[row][col];
            write_volatile(dst, *c);
        }
    }

    #[allow(clippy::volatile_composites)]
    pub fn set_line(&mut self, line: &ScreenCharLine, row: usize) {
        unsafe {
            let dst = &raw mut (*self.buffer).chars[row];
            write_volatile(dst, *line);
        }
    }

    #[allow(clippy::volatile_composites)]
    pub fn set_screen(&mut self, screen: &Screen) {
        unsafe {
            let dst = &raw mut (*self.buffer).chars;
            write_volatile(dst, *screen);
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
pub fn set_cursor(col: usize, row: usize) {
    let pos = (row * BUFFER_WIDTH + col) as u16;

    unsafe {
        outb(0xF, 0x3D4);
        outb((pos & 0xFF) as u8, 0x3D5);
        outb(0xE, 0x3D4);
        outb(((pos >> 8) & 0xFF) as u8, 0x3D5);
    }
}
