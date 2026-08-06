use crate::driver::vga::ColorCode;

pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

pub type ScreenCharLine = [ScreenChar; BUFFER_WIDTH];
pub type Screen = [ScreenCharLine; BUFFER_HEIGHT];
