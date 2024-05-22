use super::{
    history_buffer::HistoryBuffer, Color, ColorCode, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH,
    NUMBER_OF_REGULAR_TTY,
};
use crate::io::outb;
use core::fmt;
use spin::{Lazy, Mutex};

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

#[derive(Clone, Copy)]
struct TtyDescriptor {
    row_position: usize,
    column_position: usize,
    color_code: ColorCode,
}

impl TtyDescriptor {
    pub fn new() -> TtyDescriptor {
        TtyDescriptor {
            row_position: 0,
            column_position: 0,
            color_code: ColorCode::new(Color::LightGray, Color::Black),
        }
    }
}

struct Writer {
    tty_id: usize,
    tty_descriptors: [TtyDescriptor; NUMBER_OF_REGULAR_TTY],
    buffer: &'static mut Buffer,
    history: HistoryBuffer,
}

impl Writer {
    pub fn apply_byte(&mut self, byte: u8) {
        match byte {
            b'\n' | b'\r' => self.new_line(),
            b'\t' => self.write_string("    "),
            0x08 => self.backspace(),
            0x7F => self.delete(),
            byte => self.write_byte(byte),
        }
    }

    fn backspace(&mut self) {
        if self.tty_descriptors[self.tty_id].column_position > 0 {
            self.tty_descriptors[self.tty_id].column_position -= 1;
        } else if self.tty_descriptors[self.tty_id].row_position > 0 {
            self.tty_descriptors[self.tty_id].row_position -= 1;
            self.tty_descriptors[self.tty_id].column_position = BUFFER_WIDTH - 1;
        }
        self.write_ascii(b' ');
        self.update_cursor();
    }

    fn delete(&mut self) {
        self.write_ascii(b' ');
    }

    fn write_byte(&mut self, byte: u8) {
        self.write_ascii(byte);
        if self.tty_descriptors[self.tty_id].column_position + 1 >= BUFFER_WIDTH {
            self.new_line();
        } else {
            self.tty_descriptors[self.tty_id].column_position += 1;
        }
        self.update_cursor();
    }

    fn write_ascii(&mut self, byte: u8) {
        let row = self.tty_descriptors[self.tty_id].row_position;
        let col = self.tty_descriptors[self.tty_id].column_position;
        let color_code = self.tty_descriptors[self.tty_id].color_code;
        let c = ScreenChar {
            ascii_character: byte,
            color_code,
        };

        self.buffer.chars[row][col] = c;
        self.history.set_char(col, row, c);
    }

    fn new_line(&mut self) {
        if self.tty_descriptors[self.tty_id].row_position < BUFFER_HEIGHT - 1 {
            self.tty_descriptors[self.tty_id].row_position += 1;
        } else {
            self.next_line();
        }
        self.tty_descriptors[self.tty_id].column_position = 0;
        self.update_cursor();
    }

    fn old_line(&mut self) {
        if self.tty_descriptors[self.tty_id].row_position > 0 {
            self.tty_descriptors[self.tty_id].row_position -= 1;
        } else {
            self.previous_line();
        }
        self.tty_descriptors[self.tty_id].column_position = BUFFER_WIDTH - 1;
        self.update_cursor();
    }

    fn next_line(&mut self) {
        if self.history.next_line().is_ok() {
            for row in 0..BUFFER_HEIGHT {
                if let Ok(line) = self.history.get_line(row) {
                    self.buffer.chars[row] = line;
                }
            }
        } else {
            self.history.new_line();
            for row in 0..BUFFER_HEIGHT {
                if let Ok(line) = self.history.get_line(row) {
                    self.buffer.chars[row] = line;
                }
            }
        }
    }

    fn previous_line(&mut self) {
        if self.history.previous_line().is_ok() {
            for row in 0..BUFFER_HEIGHT {
                if let Ok(line) = self.history.get_line(row) {
                    self.buffer.chars[row] = line;
                }
            }
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.tty_descriptors[self.tty_id].color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = blank;
            self.history.set_char(col, row, blank);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7E | b'\n' | b'\r' | b'\t' | 0x08 | 0x7F => self.apply_byte(byte),
                _ => self.apply_byte(0xfe),
            }
        }
    }

    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.tty_descriptors[self.tty_id].row_position = 0;
        self.tty_descriptors[self.tty_id].column_position = 0;
        self.update_cursor();
    }

    pub fn cursor_right(&mut self) {
        if self.tty_descriptors[self.tty_id].column_position + 1 >= BUFFER_WIDTH {
            self.new_line();
        } else {
            self.tty_descriptors[self.tty_id].column_position += 1;
        }
        self.update_cursor();
    }

    pub fn cursor_left(&mut self) {
        if self.tty_descriptors[self.tty_id].column_position == 0 {
            self.old_line();
        } else {
            self.tty_descriptors[self.tty_id].column_position -= 1;
        }
        self.update_cursor();
    }

    pub fn cursor_down(&mut self) {
        if self.tty_descriptors[self.tty_id].row_position + 1 >= BUFFER_HEIGHT {
            let col = self.tty_descriptors[self.tty_id].column_position;
            self.new_line();
            self.tty_descriptors[self.tty_id].column_position = col;
        } else {
            self.tty_descriptors[self.tty_id].row_position += 1;
        }
        self.update_cursor();
    }

    pub fn cursor_up(&mut self) {
        if self.tty_descriptors[self.tty_id].row_position == 0 {
            let col = self.tty_descriptors[self.tty_id].column_position;
            self.old_line();
            self.tty_descriptors[self.tty_id].column_position = col;
        } else {
            self.tty_descriptors[self.tty_id].row_position -= 1;
        }
        self.update_cursor();
    }

    fn update_cursor(&self) {
        set_cursor(
            self.tty_descriptors[self.tty_id].column_position,
            self.tty_descriptors[self.tty_id].row_position,
        );
    }

    pub fn change_tty(&mut self, id: usize) {
        if self.history.change_tty(id).is_ok() {
            for row in 0..BUFFER_HEIGHT {
                if let Ok(line) = self.history.get_line(row) {
                    self.buffer.chars[row] = line;
                }
            }
            self.tty_id = id;
            self.update_cursor();
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::driver::vga::text_mode::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

static WRITER: Lazy<Mutex<Writer>> = Lazy::new(|| {
    Mutex::new(Writer {
        tty_id: 0,
        tty_descriptors: [TtyDescriptor::new(); NUMBER_OF_REGULAR_TTY],
        buffer: unsafe { &mut *(0xB8000 as *mut Buffer) },
        history: HistoryBuffer::new(),
    })
});

fn set_cursor(x: usize, y: usize) {
    let pos = (y * BUFFER_WIDTH + x) as u16;

    unsafe {
        outb(0xF, 0x3D4);
        outb((pos & 0xFF) as u8, 0x3D5);
        outb(0xE, 0x3D4);
        outb(((pos >> 8) & 0xFF) as u8, 0x3D5);
    }
}

#[inline(always)]
pub fn clear() {
    WRITER.lock().clear();
}

#[inline(always)]
pub fn cursor_right() {
    WRITER.lock().cursor_right();
}

#[inline(always)]
pub fn cursor_left() {
    WRITER.lock().cursor_left();
}

#[inline(always)]
pub fn cursor_down() {
    WRITER.lock().cursor_down();
}

#[inline(always)]
pub fn cursor_up() {
    WRITER.lock().cursor_up();
}

#[inline(always)]
pub fn change_tty(id: usize) {
    WRITER.lock().change_tty(id);
}
