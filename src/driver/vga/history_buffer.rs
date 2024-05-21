use super::{Color, ColorCode, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH, HISTORY_BUFFER_HEIGHT};

static mut BUFFER: &mut [[ScreenChar; BUFFER_WIDTH]; HISTORY_BUFFER_HEIGHT] =
    &mut [[ScreenChar {
        ascii_character: b' ',
        color_code: ColorCode::new(Color::LightGray, Color::Black),
    }; BUFFER_WIDTH]; HISTORY_BUFFER_HEIGHT];

pub struct HistoryBuffer {
    begin: usize,
    current: usize,
    end: usize,
    chars: &'static mut [[ScreenChar; BUFFER_WIDTH]; HISTORY_BUFFER_HEIGHT],
}

impl HistoryBuffer {
    pub fn new() -> HistoryBuffer {
        HistoryBuffer {
            begin: 0,
            current: 0,
            end: 0,
            chars: unsafe { BUFFER },
        }
    }

    pub fn set_char(&mut self, x: usize, y: usize, c: ScreenChar) {
        if x < BUFFER_WIDTH && y < BUFFER_HEIGHT {
            self.chars[(self.current + y) % HISTORY_BUFFER_HEIGHT][x] = c;
        }
    }

    // TODO: remove if useless
    #[allow(dead_code)]
    pub fn get_char(&self, x: usize, y: usize) -> Result<ScreenChar, ()> {
        if x < BUFFER_WIDTH && y < BUFFER_HEIGHT {
            Ok(self.chars[(self.current + y) % HISTORY_BUFFER_HEIGHT][x])
        } else {
            Err(())
        }
    }

    pub fn previous_line(&mut self) -> Result<(), ()> {
        if self.begin != self.current {
            self.current -= 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn next_line(&mut self) -> Result<(), ()> {
        if self.current != self.end {
            self.current += 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn new_line(&mut self) {
        self.end = (self.end + 1) % HISTORY_BUFFER_HEIGHT;
        self.current = self.end;
        if (self.end + BUFFER_HEIGHT - 1) % HISTORY_BUFFER_HEIGHT == self.begin {
            self.begin = (self.begin + 1) % HISTORY_BUFFER_HEIGHT;
        }
        let new_line = [ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::LightGray, Color::Black),
        }; BUFFER_WIDTH];
        self.chars[(self.end + BUFFER_HEIGHT - 1) % HISTORY_BUFFER_HEIGHT] = new_line;
    }

    pub fn get_line(&self, y: usize) -> Result<[ScreenChar; BUFFER_WIDTH], ()> {
        if y < BUFFER_HEIGHT {
            Ok(self.chars[(self.current + y) % HISTORY_BUFFER_HEIGHT])
        } else {
            Err(())
        }
    }
}
