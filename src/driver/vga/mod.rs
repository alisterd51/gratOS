mod history_buffer;
pub mod text_mode;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const HISTORY_BUFFER_HEIGHT: usize = 1000;
const NUMBER_OF_REGULAR_TTY: usize = 12;
const DEFAULT_FOREFROUND_COLOR: Color = Color::LightGray;
const DEFAULT_BACKFROUND_COLOR: Color = Color::Black;
const DEFAULT_COLOR_CODE: ColorCode =
    ColorCode::new(DEFAULT_FOREFROUND_COLOR, DEFAULT_BACKFROUND_COLOR);

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Color {
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

    fn update_foreground(&mut self, color: Color) {
        self.0 = (self.0 & 0xF0) | (color as u8);
    }

    fn update_background(&mut self, color: Color) {
        self.0 = (color as u8) << 4 | (self.0 & 0x0F);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}
