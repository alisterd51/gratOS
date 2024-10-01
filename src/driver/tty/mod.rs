mod history;

#[cfg(debug_assertions)]
use crate::{print, println};

use super::vga::{Color, ColorCode};

const HISTORY_BUFFER_HEIGHT: usize = 1000;
const NUMBER_OF_REGULAR_TTY: usize = 12;
const DEFAULT_FOREFROUND_COLOR: Color = Color::LightGray;
const DEFAULT_BACKFROUND_COLOR: Color = Color::Black;
const DEFAULT_COLOR_CODE: ColorCode =
    ColorCode::new(DEFAULT_FOREFROUND_COLOR, DEFAULT_BACKFROUND_COLOR);

// https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit
// escape sequences:

#[allow(dead_code)]
pub const CURSOR_UP: &str = "\x1B[A";
#[allow(dead_code)]
pub const CURSOR_DOWN: &str = "\x1B[B";
#[allow(dead_code)]
pub const CURSOR_RIGHT: &str = "\x1B[C";
#[allow(dead_code)]
pub const CURSOR_LEFT: &str = "\x1B[D";

#[allow(dead_code)]
pub const RESET: &str = "\x1B[0m";

#[allow(dead_code)]
pub const FG_BLACK: &str = "\x1B[30m";
#[allow(dead_code)]
pub const FG_RED: &str = "\x1B[31m";
#[allow(dead_code)]
pub const FG_GREEN: &str = "\x1B[32m";
#[allow(dead_code)]
pub const FG_YELLOW: &str = "\x1B[33m";
#[allow(dead_code)]
pub const FG_BLUE: &str = "\x1B[34m";
#[allow(dead_code)]
pub const FG_MAGENTA: &str = "\x1B[35m";
#[allow(dead_code)]
pub const FG_CYAN: &str = "\x1B[36m";
#[allow(dead_code)]
pub const FG_WHITE: &str = "\x1B[37m";
#[allow(dead_code)]
pub const FG_BRIGHT_BLACK: &str = "\x1B[90m";
#[allow(dead_code)]
pub const FG_BRIGHT_RED: &str = "\x1B[91m";
#[allow(dead_code)]
pub const FG_BRIGHT_GREEN: &str = "\x1B[92m";
#[allow(dead_code)]
pub const FG_BRIGHT_YELLOW: &str = "\x1B[93m";
#[allow(dead_code)]
pub const FG_BRIGHT_BLUE: &str = "\x1B[94m";
#[allow(dead_code)]
pub const FG_BRIGHT_MAGENTA: &str = "\x1B[95m";
#[allow(dead_code)]
pub const FG_BRIGHT_CYAN: &str = "\x1B[96m";
#[allow(dead_code)]
pub const FG_BRIGHT_WHITE: &str = "\x1B[97m";
#[allow(dead_code)]
pub const FG_RESET: &str = "\x1B[39m";

#[allow(dead_code)]
pub const BG_BLACK: &str = "\x1B[40m";
#[allow(dead_code)]
pub const BG_RED: &str = "\x1B[41m";
#[allow(dead_code)]
pub const BG_GREEN: &str = "\x1B[42m";
#[allow(dead_code)]
pub const BG_YELLOW: &str = "\x1B[43m";
#[allow(dead_code)]
pub const BG_BLUE: &str = "\x1B[44m";
#[allow(dead_code)]
pub const BG_MAGENTA: &str = "\x1B[45m";
#[allow(dead_code)]
pub const BG_CYAN: &str = "\x1B[46m";
#[allow(dead_code)]
pub const BG_WHITE: &str = "\x1B[47m";
#[allow(dead_code)]
pub const BG_BRIGHT_BLACK: &str = "\x1B[100m";
#[allow(dead_code)]
pub const BG_BRIGHT_RED: &str = "\x1B[101m";
#[allow(dead_code)]
pub const BG_BRIGHT_GREEN: &str = "\x1B[102m";
#[allow(dead_code)]
pub const BG_BRIGHT_YELLOW: &str = "\x1B[103m";
#[allow(dead_code)]
pub const BG_BRIGHT_BLUE: &str = "\x1B[104m";
#[allow(dead_code)]
pub const BG_BRIGHT_MAGENTA: &str = "\x1B[105m";
#[allow(dead_code)]
pub const BG_BRIGHT_CYAN: &str = "\x1B[106m";
#[allow(dead_code)]
pub const BG_BRIGHT_WHITE: &str = "\x1B[107m";
#[allow(dead_code)]
pub const BG_RESET: &str = "\x1B[49m";

#[cfg(debug_assertions)]
pub fn test_colors() {
    const FGS: [&str; 16] = [
        FG_BLACK,
        FG_RED,
        FG_GREEN,
        FG_YELLOW,
        FG_BLUE,
        FG_MAGENTA,
        FG_CYAN,
        FG_WHITE,
        FG_BRIGHT_BLACK,
        FG_BRIGHT_RED,
        FG_BRIGHT_GREEN,
        FG_BRIGHT_YELLOW,
        FG_BRIGHT_BLUE,
        FG_BRIGHT_MAGENTA,
        FG_BRIGHT_CYAN,
        FG_BRIGHT_WHITE,
    ];
    const BGS: [&str; 16] = [
        BG_BLACK,
        BG_RED,
        BG_GREEN,
        BG_YELLOW,
        BG_BLUE,
        BG_MAGENTA,
        BG_CYAN,
        BG_WHITE,
        BG_BRIGHT_BLACK,
        BG_BRIGHT_RED,
        BG_BRIGHT_GREEN,
        BG_BRIGHT_YELLOW,
        BG_BRIGHT_BLUE,
        BG_BRIGHT_MAGENTA,
        BG_BRIGHT_CYAN,
        BG_BRIGHT_WHITE,
    ];

    println!("test: colors escape sequences");
    println!("all:");
    for background in BGS {
        for foreground in FGS {
            print!("{}{}a{}", background, foreground, RESET);
        }
        println!();
    }
    println!("foreground reset:");
    for background in BGS {
        for foreground in FGS {
            print!("{}{}a{}b{}", background, foreground, FG_RESET, RESET);
        }
        println!();
    }
    println!("background reset:");
    for background in BGS {
        for foreground in FGS {
            print!("{}{}a{}b{}", background, foreground, BG_RESET, RESET);
        }
        println!();
    }
    println!("blue hello:");
    {
        println!("\x1B");
        println!("[");
        println!("3");
        println!("4");
        println!("m");
        println!("hello");
        println!("\x1B");
        println!("[");
        println!("0");
        println!("m");
        println!("hello");
    }
    println!("end test: colors escape sequences");
}

use crate::driver::vga::{
    text_mode::{set_cursor, Writer},
    ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH,
};
use core::fmt;
use history::History;
use spin::{Lazy, Mutex};

#[derive(Clone, Copy)]
struct TtyDescriptor {
    row_position: usize,
    column_position: usize,
    color_code: ColorCode,
}

impl TtyDescriptor {
    pub const fn new() -> TtyDescriptor {
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

struct Tty {
    escape_state: EscapeState,
    id: usize,
    descriptors: [TtyDescriptor; NUMBER_OF_REGULAR_TTY],
    writer: Writer,
    history: History,
}

impl Tty {
    fn apply_byte(&mut self, byte: u8) {
        match byte {
            b'\n' | b'\r' => self.new_line(),
            b'\t' => self.write_string("    "),
            0x08 => self.backspace(),
            0x7F => self.delete(),
            0x1B => self.escape_state = EscapeState::Esc,
            byte @ b' '..=b'~' => self.apply_escape_byte(byte),
            _ => self.write_byte(0xFE),
        }
    }

    fn apply_escape_byte(&mut self, byte: u8) {
        match self.escape_state {
            EscapeState::Normal => self.write_byte(byte),
            EscapeState::Esc => {
                self.escape_state = match byte {
                    b'[' => EscapeState::Csi(CsiParam::Undefined),
                    _ => EscapeState::Normal,
                };
            }
            EscapeState::Csi(n) => {
                self.escape_state = match byte {
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
                                    39 => self.update_foreground_color(DEFAULT_FOREFROUND_COLOR),
                                    40 => self.update_background_color(Color::Black),
                                    41 => self.update_background_color(Color::Red),
                                    42 => self.update_background_color(Color::Green),
                                    43 => self.update_background_color(Color::Brown),
                                    44 => self.update_background_color(Color::Blue),
                                    45 => self.update_background_color(Color::Magenta),
                                    46 => self.update_background_color(Color::Cyan),
                                    47 => self.update_background_color(Color::LightGray),
                                    49 => self.update_background_color(DEFAULT_BACKFROUND_COLOR),
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
                            let digit = u32::from(byte - b'0');
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
        }
    }

    fn backspace(&mut self) {
        if self.descriptors[self.id].column_position > 0 {
            self.descriptors[self.id].column_position -= 1;
        } else if self.descriptors[self.id].row_position > 0 {
            self.descriptors[self.id].row_position -= 1;
            self.descriptors[self.id].column_position = BUFFER_WIDTH - 1;
        }
        self.write_ascii(b' ');
        self.update_cursor();
    }

    fn delete(&mut self) {
        self.write_ascii(b' ');
    }

    fn write_byte(&mut self, byte: u8) {
        self.write_ascii(byte);
        if self.descriptors[self.id].column_position + 1 >= BUFFER_WIDTH {
            self.new_line();
        } else {
            self.descriptors[self.id].column_position += 1;
        }
        self.update_cursor();
    }

    fn write_ascii(&mut self, byte: u8) {
        let row = self.descriptors[self.id].row_position;
        let col = self.descriptors[self.id].column_position;
        let color_code = self.descriptors[self.id].color_code;
        let c = ScreenChar {
            ascii_character: byte,
            color_code,
        };

        self.writer.set_char(&c, col, row);
        self.history.set_char(&c, col, row);
    }

    fn new_line(&mut self) {
        if self.descriptors[self.id].row_position < BUFFER_HEIGHT - 1 {
            self.descriptors[self.id].row_position += 1;
        } else {
            self.next_line();
        }
        self.descriptors[self.id].column_position = 0;
        self.update_cursor();
    }

    fn old_line(&mut self) {
        if self.descriptors[self.id].row_position > 0 {
            self.descriptors[self.id].row_position -= 1;
        } else {
            self.previous_line();
        }
        self.descriptors[self.id].column_position = BUFFER_WIDTH - 1;
        self.update_cursor();
    }

    fn next_line(&mut self) {
        if self.history.next_line().is_ok() {
            let screen = self.history.get_screen();
            self.writer.set_screen(&screen);
        } else {
            self.history.new_line();
            let screen = self.history.get_screen();
            self.writer.set_screen(&screen);
        }
    }

    fn previous_line(&mut self) {
        if self.history.previous_line().is_ok() {
            let screen = self.history.get_screen();
            self.writer.set_screen(&screen);
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.descriptors[self.id].color_code,
        };
        let blank_line = [blank; BUFFER_WIDTH];

        self.writer.set_line(&blank_line, row);
        self.history.set_line(&blank_line, row);
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
        self.descriptors[self.id].row_position = 0;
        self.descriptors[self.id].column_position = 0;
        self.update_cursor();
    }

    pub fn cursor_right(&mut self) {
        if self.descriptors[self.id].column_position + 1 >= BUFFER_WIDTH {
            self.new_line();
        } else {
            self.descriptors[self.id].column_position += 1;
        }
        self.update_cursor();
    }

    pub fn cursor_left(&mut self) {
        if self.descriptors[self.id].column_position == 0 {
            self.old_line();
        } else {
            self.descriptors[self.id].column_position -= 1;
        }
        self.update_cursor();
    }

    pub fn cursor_down(&mut self) {
        if self.descriptors[self.id].row_position + 1 >= BUFFER_HEIGHT {
            let col = self.descriptors[self.id].column_position;
            self.new_line();
            self.descriptors[self.id].column_position = col;
        } else {
            self.descriptors[self.id].row_position += 1;
        }
        self.update_cursor();
    }

    pub fn cursor_up(&mut self) {
        if self.descriptors[self.id].row_position == 0 {
            let col = self.descriptors[self.id].column_position;
            self.old_line();
            self.descriptors[self.id].column_position = col;
        } else {
            self.descriptors[self.id].row_position -= 1;
        }
        self.update_cursor();
    }

    fn update_cursor(&self) {
        set_cursor(
            self.descriptors[self.id].column_position,
            self.descriptors[self.id].row_position,
        );
    }

    pub fn change_tty_id(&mut self, id: usize) {
        if self.history.change_tty_id(id).is_ok() {
            let screen = self.history.get_screen();
            self.writer.set_screen(&screen);
            self.id = id;
            self.update_cursor();
        }
    }

    fn update_color(&mut self, color_code: ColorCode) {
        self.descriptors[self.id].color_code = color_code;
    }

    pub fn update_foreground_color(&mut self, color: Color) {
        self.descriptors[self.id].color_code.set_foreground(color);
    }

    pub fn update_background_color(&mut self, color: Color) {
        self.descriptors[self.id].color_code.set_background(color);
    }
}

impl fmt::Write for Tty {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        #[allow(clippy::used_underscore_items)]
        $crate::driver::tty::_print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::print!("{}\n", format_args!($($arg)*));
    }};
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

static WRITER: Lazy<Mutex<Tty>> = Lazy::new(|| {
    Mutex::new(Tty {
        escape_state: EscapeState::Normal,
        id: 0,
        descriptors: [TtyDescriptor::new(); NUMBER_OF_REGULAR_TTY],
        writer: Writer::new(),
        history: History::new(),
    })
});

#[allow(clippy::inline_always)]
#[inline(always)]
pub fn clear() {
    WRITER.lock().clear();
}

#[allow(clippy::inline_always)]
#[inline(always)]
pub fn change_tty_id(id: usize) {
    WRITER.lock().change_tty_id(id);
}
