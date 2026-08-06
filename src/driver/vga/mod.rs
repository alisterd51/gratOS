mod buffer;
mod color;

pub mod text_mode;

pub use buffer::{BUFFER_HEIGHT, BUFFER_WIDTH, Screen, ScreenChar, ScreenCharLine};
pub use color::{Color, ColorCode};
