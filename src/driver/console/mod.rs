mod history;

use super::{
    shell,
    vga::{
        BUFFER_HEIGHT, BUFFER_WIDTH, Color, ColorCode, Line, ScreenChar,
        text_mode::{Writer, set_cursor},
    },
};
use crate::mutex::Mutex;
use core::fmt;
use history::History;

pub const NUMBER_OF_REGULAR_TTY: usize = 12;
const HISTORY_BUFFER_HEIGHT: usize = 1000;
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
pub const SCROLL_UP: &str = "\x1B[S";
#[allow(dead_code)]
pub const SCROLL_DOWN: &str = "\x1B[T";

pub const RESET: &str = "\x1B[0m";

pub const FG_BLACK: &str = "\x1B[30m";
pub const FG_RED: &str = "\x1B[31m";
pub const FG_GREEN: &str = "\x1B[32m";
pub const FG_YELLOW: &str = "\x1B[33m";
pub const FG_BLUE: &str = "\x1B[34m";
pub const FG_MAGENTA: &str = "\x1B[35m";
pub const FG_CYAN: &str = "\x1B[36m";
pub const FG_WHITE: &str = "\x1B[37m";
pub const FG_BRIGHT_BLACK: &str = "\x1B[90m";
pub const FG_BRIGHT_RED: &str = "\x1B[91m";
pub const FG_BRIGHT_GREEN: &str = "\x1B[92m";
pub const FG_BRIGHT_YELLOW: &str = "\x1B[93m";
pub const FG_BRIGHT_BLUE: &str = "\x1B[94m";
pub const FG_BRIGHT_MAGENTA: &str = "\x1B[95m";
pub const FG_BRIGHT_CYAN: &str = "\x1B[96m";
pub const FG_BRIGHT_WHITE: &str = "\x1B[97m";
pub const FG_RESET: &str = "\x1B[39m";

pub const BG_BLACK: &str = "\x1B[40m";
pub const BG_RED: &str = "\x1B[41m";
pub const BG_GREEN: &str = "\x1B[42m";
pub const BG_YELLOW: &str = "\x1B[43m";
pub const BG_BLUE: &str = "\x1B[44m";
pub const BG_MAGENTA: &str = "\x1B[45m";
pub const BG_CYAN: &str = "\x1B[46m";
pub const BG_WHITE: &str = "\x1B[47m";
pub const BG_BRIGHT_BLACK: &str = "\x1B[100m";
pub const BG_BRIGHT_RED: &str = "\x1B[101m";
pub const BG_BRIGHT_GREEN: &str = "\x1B[102m";
pub const BG_BRIGHT_YELLOW: &str = "\x1B[103m";
pub const BG_BRIGHT_BLUE: &str = "\x1B[104m";
pub const BG_BRIGHT_MAGENTA: &str = "\x1B[105m";
pub const BG_BRIGHT_CYAN: &str = "\x1B[106m";
pub const BG_BRIGHT_WHITE: &str = "\x1B[107m";
pub const BG_RESET: &str = "\x1B[49m";

#[derive(Clone, Copy)]
struct ConsoleDescriptor {
    row_position: usize,
    column_position: usize,
    color_code: ColorCode,
}

impl ConsoleDescriptor {
    pub const fn new() -> Self {
        Self {
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

struct Console {
    escape_state: EscapeState,
    id: usize,
    descriptors: [ConsoleDescriptor; NUMBER_OF_REGULAR_TTY],
    writer: Writer,
    history: History,
}

impl Console {
    const fn new() -> Self {
        Self {
            escape_state: EscapeState::Normal,
            id: 0,
            descriptors: [ConsoleDescriptor::new(); NUMBER_OF_REGULAR_TTY],
            writer: Writer::new(),
            history: History::new(),
        }
    }

    fn apply_byte(&mut self, byte: u8) {
        match byte {
            b'\n' | b'\r' => {
                if self.history.end_line().is_ok() {
                    self.update_screen();
                }
                let line = self.get_current_line();
                shell::add_line_to_buf(self.id, &line);
                self.new_line();
            }
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
                            byte @ b'A'..=b'Z' => {
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
                                        b'S' => self.scroll_up(),
                                        b'T' => self.scroll_down(),
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
        if self.history.end_line().is_ok() {
            self.update_screen();
        }
        if self.descriptors[self.id].column_position > shell::ps1().len() {
            self.descriptors[self.id].column_position -= 1;
            self.write_ascii(b' ');
            self.update_cursor();
        }
    }

    fn delete(&mut self) {
        if self.history.end_line().is_ok() {
            self.update_screen();
        }
        self.write_ascii(b' ');
    }

    fn write_byte(&mut self, byte: u8) {
        if self.history.end_line().is_ok() {
            self.update_screen();
        }
        if self.descriptors[self.id].column_position + 1 < BUFFER_WIDTH {
            self.write_ascii(byte);
            self.descriptors[self.id].column_position += 1;
        }
        self.update_cursor();
    }

    const fn write_ascii(&mut self, byte: u8) {
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

    // wait `for_each` in `const fn` <https://github.com/rust-lang/rust/issues/67792>
    fn get_current_line(&self) -> Line {
        let line = self
            .history
            .get_line(self.descriptors[self.id].row_position);
        let mut new_line = [0u8; BUFFER_WIDTH];

        if let Ok(line) = line {
            new_line
                .iter_mut()
                .zip(line.iter())
                .for_each(|(new_char, screen_char)| {
                    let c = screen_char.ascii_character;
                    if c.is_ascii_whitespace() {
                        *new_char = b'\0';
                    } else {
                        *new_char = c;
                    }
                });
        }
        new_line
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

    fn update_screen(&mut self) {
        let screen = self.history.get_screen();
        self.writer.set_screen(&screen);
    }

    fn next_line(&mut self) {
        if self.history.next_line().is_err() {
            self.history.new_line();
        }
        self.update_screen();
    }

    const fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.descriptors[self.id].color_code,
        };
        let blank_line = [blank; BUFFER_WIDTH];

        self.writer.set_line(&blank_line, row);
        self.history.set_line(&blank_line, row);
    }

    fn write_string(&mut self, s: &str) {
        s.bytes().for_each(|byte| self.apply_byte(byte));
    }

    fn clear(&mut self) {
        (0..BUFFER_HEIGHT).for_each(|row| self.clear_row(row));
        self.descriptors[self.id].row_position = 0;
        self.descriptors[self.id].column_position = 0;
        self.update_cursor();
    }

    const fn cursor_right(&self) {
        shell::right(self.id);
    }

    const fn cursor_left(&self) {
        shell::left(self.id);
    }

    const fn cursor_down(&self) {
        shell::down(self.id);
    }

    const fn cursor_up(&self) {
        shell::up(self.id);
    }

    fn update_cursor(&self) {
        set_cursor(
            self.descriptors[self.id].column_position,
            self.descriptors[self.id].row_position,
        );
    }

    fn scroll_up(&mut self) {
        if self.history.previous_line().is_ok() {
            self.update_screen();
        }
    }

    fn scroll_down(&mut self) {
        if self.history.next_line().is_ok() {
            self.update_screen();
        }
    }

    fn change_tty_id(&mut self, id: usize) {
        if self.history.change_tty_id(id).is_ok() {
            self.update_screen();
            self.id = id;
            self.update_cursor();
        }
    }

    const fn update_color(&mut self, color_code: ColorCode) {
        self.descriptors[self.id].color_code = color_code;
    }

    const fn update_foreground_color(&mut self, color: Color) {
        self.descriptors[self.id].color_code.set_foreground(color);
    }

    const fn update_background_color(&mut self, color: Color) {
        self.descriptors[self.id].color_code.set_background(color);
    }

    const fn get_tty_id(&self) -> usize {
        self.id
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        #[allow(clippy::used_underscore_items)]
        $crate::driver::console::_print(format_args!($($arg)*));
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

static WRITER: Mutex<Console> = Mutex::new(Console::new());

pub fn clear() {
    WRITER.lock().clear();
}

pub fn change_tty_id(id: usize) {
    WRITER.lock().change_tty_id(id);
}

pub fn get_tty_id() -> usize {
    WRITER.lock().get_tty_id()
}

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
            print!("{background}{foreground}a{RESET}");
        }
        println!();
    }
    println!("foreground reset:");
    for background in BGS {
        for foreground in FGS {
            print!("{background}{foreground}a{FG_RESET}b{RESET}");
        }
        println!();
    }
    println!("background reset:");
    for background in BGS {
        for foreground in FGS {
            print!("{background}{foreground}a{BG_RESET}b{RESET}");
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
