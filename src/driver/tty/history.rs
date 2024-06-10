use crate::driver::vga::{ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH};

use super::{DEFAULT_COLOR_CODE, HISTORY_BUFFER_HEIGHT, NUMBER_OF_REGULAR_TTY};

static mut BUFFER: &mut [[[ScreenChar; BUFFER_WIDTH]; HISTORY_BUFFER_HEIGHT];
         NUMBER_OF_REGULAR_TTY] = &mut [[[ScreenChar {
    ascii_character: b' ',
    color_code: DEFAULT_COLOR_CODE,
}; BUFFER_WIDTH]; HISTORY_BUFFER_HEIGHT];
    NUMBER_OF_REGULAR_TTY];

#[derive(Clone, Copy)]
struct HistoryDescriptor {
    pub begin: usize,
    pub current: usize,
    pub end: usize,
}

impl HistoryDescriptor {
    pub const fn new() -> HistoryDescriptor {
        HistoryDescriptor {
            begin: 0,
            current: 0,
            end: 0,
        }
    }
}

pub struct History {
    tty_id: usize,
    history_descriptors: [HistoryDescriptor; NUMBER_OF_REGULAR_TTY],
    chars:
        &'static mut [[[ScreenChar; BUFFER_WIDTH]; HISTORY_BUFFER_HEIGHT]; NUMBER_OF_REGULAR_TTY],
}

impl History {
    pub fn new() -> History {
        History {
            tty_id: 0,
            history_descriptors: [HistoryDescriptor::new(); NUMBER_OF_REGULAR_TTY],
            chars: unsafe { BUFFER },
        }
    }

    pub fn set_char(&mut self, x: usize, y: usize, c: ScreenChar) {
        if x < BUFFER_WIDTH && y < BUFFER_HEIGHT {
            self.chars[self.tty_id]
                [(self.history_descriptors[self.tty_id].current + y) % HISTORY_BUFFER_HEIGHT][x] =
                c;
        }
    }

    pub fn set_line(&mut self, line: [ScreenChar; BUFFER_WIDTH], y: usize) {
        self.chars[self.tty_id]
            [(self.history_descriptors[self.tty_id].current + y) % HISTORY_BUFFER_HEIGHT] = line;
    }

    // TODO: remove if useless
    #[allow(dead_code)]
    pub fn get_char(&self, x: usize, y: usize) -> Result<ScreenChar, ()> {
        if x < BUFFER_WIDTH && y < BUFFER_HEIGHT {
            Ok(self.chars[self.tty_id]
                [(self.history_descriptors[self.tty_id].current + y) % HISTORY_BUFFER_HEIGHT][x])
        } else {
            Err(())
        }
    }

    pub fn get_line(&self, y: usize) -> Result<[ScreenChar; BUFFER_WIDTH], ()> {
        if y < BUFFER_HEIGHT {
            Ok(self.chars[self.tty_id]
                [(self.history_descriptors[self.tty_id].current + y) % HISTORY_BUFFER_HEIGHT])
        } else {
            Err(())
        }
    }

    pub fn previous_line(&mut self) -> Result<(), ()> {
        if self.history_descriptors[self.tty_id].begin
            != self.history_descriptors[self.tty_id].current
        {
            self.history_descriptors[self.tty_id].current -= 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn next_line(&mut self) -> Result<(), ()> {
        if self.history_descriptors[self.tty_id].current
            != self.history_descriptors[self.tty_id].end
        {
            self.history_descriptors[self.tty_id].current += 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn new_line(&mut self) {
        self.history_descriptors[self.tty_id].end =
            (self.history_descriptors[self.tty_id].end + 1) % HISTORY_BUFFER_HEIGHT;
        self.history_descriptors[self.tty_id].current = self.history_descriptors[self.tty_id].end;
        if (self.history_descriptors[self.tty_id].end + BUFFER_HEIGHT - 1) % HISTORY_BUFFER_HEIGHT
            == self.history_descriptors[self.tty_id].begin
        {
            self.history_descriptors[self.tty_id].begin =
                (self.history_descriptors[self.tty_id].begin + 1) % HISTORY_BUFFER_HEIGHT;
        }
        let new_line = [ScreenChar {
            ascii_character: b' ',
            color_code: DEFAULT_COLOR_CODE,
        }; BUFFER_WIDTH];
        self.chars[self.tty_id][(self.history_descriptors[self.tty_id].end + BUFFER_HEIGHT - 1)
            % HISTORY_BUFFER_HEIGHT] = new_line;
    }

    pub fn change_tty(&mut self, id: usize) -> Result<(), ()> {
        if id < NUMBER_OF_REGULAR_TTY && self.tty_id != id {
            self.tty_id = id;
            Ok(())
        } else {
            Err(())
        }
    }
}
