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

#[derive(Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> Self {
        Self(((background as u8) << 4) | (foreground as u8))
    }

    pub const fn set_foreground(&mut self, color: Color) {
        self.0 = (self.0 & 0xF0) | (color as u8);
    }

    pub const fn set_background(&mut self, color: Color) {
        self.0 = ((color as u8) << 4) | (self.0 & 0x0F);
    }
}
