use super::{
    history_buffer::HistoryBuffer, Color, ColorCode, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH,
    DEFAULT_BACKFROUND_COLOR, DEFAULT_COLOR_CODE, DEFAULT_FOREFROUND_COLOR, NUMBER_OF_REGULAR_TTY,
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
            color_code: DEFAULT_COLOR_CODE,
        }
    }
}

#[derive(Clone, Copy)]
enum CsiParam {
    Undefined,
    Defined(u32),
    Invalid,
}

enum EscapeState {
    Normal,
    Esc,
    Csi(CsiParam),
}

struct Writer {
    escape_state: EscapeState,
    tty_id: usize,
    tty_descriptors: [TtyDescriptor; NUMBER_OF_REGULAR_TTY],
    buffer: &'static mut Buffer,
    history: HistoryBuffer,
}

impl Writer {
    fn apply_byte(&mut self, byte: u8) {
        match byte {
            b'\n' | b'\r' => self.new_line(),
            b'\t' => self.write_string("    "),
            0x08 => self.backspace(),
            0x7F => self.delete(),
            0x1B => self.escape_state = EscapeState::Esc,
            byte @ b' '..=b'~' => match self.escape_state {
                EscapeState::Normal => self.write_byte(byte),
                EscapeState::Esc => {
                    self.escape_state = match byte {
                        b'[' => EscapeState::Csi(CsiParam::Undefined),
                        _ => EscapeState::Normal,
                    };
                }
                EscapeState::Csi(n) => {
                    self.escape_state =
                        match byte {
                            byte @ b'@'..=b'~' => {
                                match byte {
                                    byte @ b'A'..=b'D' => {
                                        let mut n = match n {
                                            CsiParam::Defined(n) => n,
                                            CsiParam::Undefined => 1,
                                            CsiParam::Invalid => 0,
                                        };
                                        while n > 0 {
                                            match byte {
                                                b'A' => self.cursor_up(),
                                                b'B' => self.cursor_down(),
                                                b'C' => self.cursor_right(),
                                                b'D' => self.cursor_left(),
                                                _ => {}
                                            }
                                            n -= 1;
                                        }
                                    }
                                    b'm' => match n {
                                        CsiParam::Undefined => {
                                            self.update_color(DEFAULT_COLOR_CODE);
                                        }
                                        CsiParam::Defined(n) => match n {
                                            0 => self.update_color(DEFAULT_COLOR_CODE),
                                            30 => self.update_foreground_color(Color::Black),
                                            31 => self.update_foreground_color(Color::Red),
                                            32 => self.update_foreground_color(Color::Green),
                                            33 => self.update_foreground_color(Color::Brown),
                                            34 => self.update_foreground_color(Color::Blue),
                                            35 => self.update_foreground_color(Color::Magenta),
                                            36 => self.update_foreground_color(Color::Cyan),
                                            37 => self.update_foreground_color(Color::LightGray),
                                            39 => self
                                                .update_foreground_color(DEFAULT_FOREFROUND_COLOR),
                                            40 => self.update_background_color(Color::Black),
                                            41 => self.update_background_color(Color::Red),
                                            42 => self.update_background_color(Color::Green),
                                            43 => self.update_background_color(Color::Brown),
                                            44 => self.update_background_color(Color::Blue),
                                            45 => self.update_background_color(Color::Magenta),
                                            46 => self.update_background_color(Color::Cyan),
                                            47 => self.update_background_color(Color::LightGray),
                                            49 => self
                                                .update_background_color(DEFAULT_BACKFROUND_COLOR),
                                            90 => self.update_foreground_color(Color::DarkGray),
                                            91 => self.update_foreground_color(Color::LightRed),
                                            92 => self.update_foreground_color(Color::LightGreen),
                                            93 => self.update_foreground_color(Color::Yellow),
                                            94 => self.update_foreground_color(Color::LightBlue),
                                            95 => self.update_foreground_color(Color::Pink),
                                            96 => self.update_foreground_color(Color::LightCyan),
                                            97 => self.update_foreground_color(Color::White),
                                            100 => self.update_background_color(Color::DarkGray),
                                            101 => self.update_background_color(Color::LightRed),
                                            102 => self.update_background_color(Color::LightGreen),
                                            103 => self.update_background_color(Color::Yellow),
                                            104 => self.update_background_color(Color::LightBlue),
                                            105 => self.update_background_color(Color::Pink),
                                            106 => self.update_background_color(Color::LightCyan),
                                            107 => self.update_background_color(Color::White),
                                            _ => {}
                                        },
                                        CsiParam::Invalid => {}
                                    },
                                    _ => {}
                                }
                                EscapeState::Normal
                            }
                            byte => match byte {
                                byte @ b'0'..=b'9' => {
                                    let digit = (byte - b'0') as u32;
                                    let csi_param = match n {
                                        CsiParam::Undefined => {
                                            if digit == 0 {
                                                CsiParam::Undefined
                                            } else {
                                                CsiParam::Defined(digit)
                                            }
                                        }
                                        CsiParam::Defined(n) => CsiParam::Defined(n * 10 + digit),
                                        CsiParam::Invalid => CsiParam::Invalid,
                                    };
                                    EscapeState::Csi(csi_param)
                                }
                                _ => EscapeState::Csi(CsiParam::Invalid),
                            },
                        };
                }
            },
            _ => self.write_byte(0xFE),
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

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.apply_byte(byte);
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

    fn update_color(&mut self, color_code: ColorCode) {
        self.tty_descriptors[self.tty_id].color_code = color_code;
    }

    pub fn update_foreground_color(&mut self, color: Color) {
        self.tty_descriptors[self.tty_id]
            .color_code
            .update_foreground(color);
    }

    pub fn update_background_color(&mut self, color: Color) {
        self.tty_descriptors[self.tty_id]
            .color_code
            .update_background(color);
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
        escape_state: EscapeState::Normal,
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
pub fn change_tty(id: usize) {
    WRITER.lock().change_tty(id);
}
