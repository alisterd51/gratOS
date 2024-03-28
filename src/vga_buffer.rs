use crate::io::outb;
use core::{fmt, str::Bytes};
use lazy_static::lazy_static;
use spin::Mutex;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }

    fn set_foreground(&mut self, foreground: Color) {
        self.0 = self.0 & 0xF0 | foreground as u8;
    }

    fn set_background(&mut self, background: Color) {
        self.0 = self.0 & 0x0F | (background as u8) << 4;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    row_position: usize,
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    // REMIND: Change these constants to variables when we implement the ability to change the
    // default values.
    const DEFAULT_BACKGROUND_COLOR: Color = Color::Black;
    const DEFAULT_FOREGROUND_COLOR: Color = Color::LightGray;

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            0x0A => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;
                let color_code = self.color_code;

                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
                self.update_cursor();
            }
        }
    }

    fn new_line(&mut self) {
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            self.scroll_down();
        }
        self.column_position = 0;
        self.update_cursor();
    }

    fn scroll_down(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            let row_above = row - 1;

            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col];

                self.buffer.chars[row_above][col] = character;
            }
        }
        self.clear_now(BUFFER_HEIGHT - 1);
    }

    // REMIND: incorrect name, should be `clear_row`
    fn clear_now(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: 0x20,
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }

    pub fn write_string(&mut self, s: &str) {
        let mut bytes = s.bytes();

        while let Some(byte) = bytes.next() {
            match byte {
                // newline
                0x0A => self.new_line(),
                // escaped sequence
                0x1B => self.interpret_escape_sequence(&mut bytes),
                // printable ASCII byte
                0x20..=0x7E => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xFE),
            }
        }
    }

    fn interpret_escape_sequence(&mut self, bytes: &mut Bytes<'_>) {
        if let Some(byte) = bytes.next() {
            match byte {
                0x5B => self.interpret_csi_sequence(bytes),
                _ => panic!("unknown escape sequence"),
            }
        } else {
            panic!("escape sequence ended unexpectedly");
        }
    }

    #[allow(clippy::single_match)]
    fn interpret_csi_sequence(&mut self, bytes: &mut Bytes<'_>) {
        const MAXIMUM_LENGTH: usize = 2;

        let mut parameters: [u8; MAXIMUM_LENGTH] = [0; MAXIMUM_LENGTH];

        for (i, byte) in bytes.enumerate() {
            match byte {
                0x6D => {
                    self.interpret_srg_parameters(&parameters);
                    break;
                }
                _ => (),
            }
            if i >= MAXIMUM_LENGTH {
                panic!("CSI sequence too long");
            }
            parameters[i] = byte;
        }
    }

    fn interpret_srg_parameters(&mut self, parameters: &[u8]) {
        match parameters {
            [0x30, 0x00] => {
                self.color_code
                    .set_foreground(Self::DEFAULT_FOREGROUND_COLOR);
                self.color_code
                    .set_background(Self::DEFAULT_BACKGROUND_COLOR);
            }
            [0x33, 0x30] => self.color_code.set_foreground(Color::Black),
            [0x33, 0x31] => self.color_code.set_foreground(Color::Red),
            [0x33, 0x32] => self.color_code.set_foreground(Color::Green),
            [0x33, 0x33] => self.color_code.set_foreground(Color::Yellow),
            [0x33, 0x34] => self.color_code.set_foreground(Color::Blue),
            [0x33, 0x35] => self.color_code.set_foreground(Color::Magenta),
            [0x33, 0x36] => self.color_code.set_foreground(Color::Cyan),
            [0x33, 0x37] => self.color_code.set_foreground(Color::White),
            [0x33, 0x39] => self
                .color_code
                .set_foreground(Self::DEFAULT_FOREGROUND_COLOR),
            [0x34, 0x30] => self.color_code.set_background(Color::Black),
            [0x34, 0x31] => self.color_code.set_background(Color::Red),
            [0x34, 0x32] => self.color_code.set_background(Color::Green),
            [0x34, 0x33] => self.color_code.set_background(Color::Yellow),
            [0x34, 0x34] => self.color_code.set_background(Color::Blue),
            [0x34, 0x35] => self.color_code.set_background(Color::Magenta),
            [0x34, 0x36] => self.color_code.set_background(Color::Cyan),
            [0x34, 0x37] => self.color_code.set_background(Color::White),
            [0x34, 0x39] => self
                .color_code
                .set_background(Self::DEFAULT_BACKGROUND_COLOR),
            _ => panic!("unknown SRG parameters"),
        }
    }

    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_now(row);
        }
        self.row_position = 0;
        self.column_position = 0;
        self.update_cursor();
    }

    fn update_cursor(&self) {
        set_cursor(self.column_position, self.row_position);
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
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;

    WRITER.lock().write_fmt(args).unwrap();
}

lazy_static! {
    static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        row_position: 0,
        column_position: 0,
        color_code: ColorCode::new(
            Writer::DEFAULT_FOREGROUND_COLOR,
            Writer::DEFAULT_BACKGROUND_COLOR
        ),
        buffer: unsafe { &mut *(0xB8000 as *mut Buffer) },
    });
}

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
