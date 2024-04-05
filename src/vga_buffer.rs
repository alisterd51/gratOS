use crate::io::outb;
use core::{fmt, slice::Iter};
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

    fn write_bytes(&mut self, bytes: &[u8]) {
        let mut bytes = bytes.iter();

        while let Some(byte) = bytes.next() {
            match byte {
                // printable ASCII byte
                b' '..=b'~' => self.write_byte(*byte),
                // newline
                b'\n' => self.new_line(),
                // escaped sequence
                0x1B => self.interpret_escape_sequence(&mut bytes),
                // not part of printable ASCII range
                _ => self.write_byte(0xFE),
            }
        }
    }

    fn write_byte(&mut self, byte: u8) {
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
        self.update_cursor_position();
    }

    fn new_line(&mut self) {
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            self.scroll_down();
        }
        self.column_position = 0;
        self.update_cursor_position();
    }

    fn scroll_down(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            let row_above = row - 1;

            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col];

                self.buffer.chars[row_above][col] = character;
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }

    fn interpret_escape_sequence(&mut self, bytes: &mut Iter<u8>) {
        if let Some(byte) = bytes.next() {
            match byte {
                b'[' => self.interpret_csi_sequence(bytes),
                _ => panic!("unknown escape sequence"),
            }
        } else {
            panic!("escape sequence ended unexpectedly");
        }
    }

    #[allow(clippy::single_match)]
    fn interpret_csi_sequence(&mut self, bytes: &mut Iter<u8>) {
        const MAXIMUM_LENGTH: usize = 2;

        let mut parameters = [0; MAXIMUM_LENGTH];

        for (i, byte) in bytes.enumerate() {
            match byte {
                b'm' => {
                    self.interpret_srg_parameters(&parameters);
                    return;
                }
                _ => (),
            }
            if i >= MAXIMUM_LENGTH {
                panic!("CSI sequence too long");
            }
            parameters[i] = *byte;
        }
        panic!("unknown CSI sequence");
    }

    fn interpret_srg_parameters(&mut self, parameters: &[u8]) {
        match parameters {
            [b'0', 0x00] => {
                self.color_code
                    .set_foreground(Self::DEFAULT_FOREGROUND_COLOR);
                self.color_code
                    .set_background(Self::DEFAULT_BACKGROUND_COLOR);
            }
            [b'3', b'0'] => self.color_code.set_foreground(Color::Black),
            [b'3', b'1'] => self.color_code.set_foreground(Color::Red),
            [b'3', b'2'] => self.color_code.set_foreground(Color::Green),
            [b'3', b'3'] => self.color_code.set_foreground(Color::Yellow),
            [b'3', b'4'] => self.color_code.set_foreground(Color::Blue),
            [b'3', b'5'] => self.color_code.set_foreground(Color::Magenta),
            [b'3', b'6'] => self.color_code.set_foreground(Color::Cyan),
            [b'3', b'7'] => self.color_code.set_foreground(Color::White),
            [b'3', b'9'] => self
                .color_code
                .set_foreground(Self::DEFAULT_FOREGROUND_COLOR),
            [b'4', b'0'] => self.color_code.set_background(Color::Black),
            [b'4', b'1'] => self.color_code.set_background(Color::Red),
            [b'4', b'2'] => self.color_code.set_background(Color::Green),
            [b'4', b'3'] => self.color_code.set_background(Color::Yellow),
            [b'4', b'4'] => self.color_code.set_background(Color::Blue),
            [b'4', b'5'] => self.color_code.set_background(Color::Magenta),
            [b'4', b'6'] => self.color_code.set_background(Color::Cyan),
            [b'4', b'7'] => self.color_code.set_background(Color::White),
            [b'4', b'9'] => self
                .color_code
                .set_background(Self::DEFAULT_BACKGROUND_COLOR),
            _ => panic!("unknown SRG parameters"),
        }
    }

    fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.row_position = 0;
        self.column_position = 0;
        self.update_cursor_position();
    }

    fn update_cursor_position(&self) {
        set_cursor_position(self.column_position, self.row_position);
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_bytes(s.as_bytes());
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

fn set_cursor_position(x: usize, y: usize) {
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
