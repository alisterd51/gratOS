use crate::driver::vga::{Line, Screen, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH};

use super::{DEFAULT_COLOR_CODE, HISTORY_BUFFER_HEIGHT, NUMBER_OF_REGULAR_TTY};

static mut BUFFER: *mut [[Line; HISTORY_BUFFER_HEIGHT]; NUMBER_OF_REGULAR_TTY] =
    &mut [[[ScreenChar {
        ascii_character: b' ',
        color_code: DEFAULT_COLOR_CODE,
    }; BUFFER_WIDTH]; HISTORY_BUFFER_HEIGHT]; NUMBER_OF_REGULAR_TTY];

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
    descriptors: [HistoryDescriptor; NUMBER_OF_REGULAR_TTY],
    chars: *mut [[Line; HISTORY_BUFFER_HEIGHT]; NUMBER_OF_REGULAR_TTY],
}

unsafe impl Send for History {}

impl History {
    pub fn new() -> History {
        History {
            tty_id: 0,
            descriptors: [HistoryDescriptor::new(); NUMBER_OF_REGULAR_TTY],
            chars: unsafe { BUFFER },
        }
    }

    pub fn set_char(&mut self, c: &ScreenChar, col: usize, row: usize) {
        if col < BUFFER_WIDTH && row < BUFFER_HEIGHT {
            unsafe {
                (*self.chars)[self.tty_id]
                    [(self.descriptors[self.tty_id].current + row) % HISTORY_BUFFER_HEIGHT][col] =
                    *c;
            }
        }
    }

    pub fn set_line(&mut self, line: &Line, row: usize) {
        unsafe {
            (*self.chars)[self.tty_id]
                [(self.descriptors[self.tty_id].current + row) % HISTORY_BUFFER_HEIGHT] = *line;
        }
    }

    // TODO: remove if useless
    #[allow(dead_code)]
    pub fn get_char(&self, x: usize, y: usize) -> Result<ScreenChar, ()> {
        if x < BUFFER_WIDTH && y < BUFFER_HEIGHT {
            Ok(unsafe {
                (*self.chars)[self.tty_id]
                    [(self.descriptors[self.tty_id].current + y) % HISTORY_BUFFER_HEIGHT][x]
            })
        } else {
            Err(())
        }
    }

    // TODO: remove if useless
    #[allow(dead_code)]
    pub fn get_line(&self, y: usize) -> Result<Line, ()> {
        if y < BUFFER_HEIGHT {
            Ok(unsafe {
                (*self.chars)[self.tty_id]
                    [(self.descriptors[self.tty_id].current + y) % HISTORY_BUFFER_HEIGHT]
            })
        } else {
            Err(())
        }
    }

    pub fn get_screen(&self) -> Screen {
        let mut screen = [[ScreenChar {
            ascii_character: b' ',
            color_code: DEFAULT_COLOR_CODE,
        }; BUFFER_WIDTH]; BUFFER_HEIGHT];
        for (i, s) in screen.iter_mut().enumerate() {
            unsafe {
                *s = (*self.chars)[self.tty_id]
                    [(self.descriptors[self.tty_id].current + i) % HISTORY_BUFFER_HEIGHT];
            }
        }
        screen
    }

    pub fn previous_line(&mut self) -> Result<(), ()> {
        if self.descriptors[self.tty_id].begin == self.descriptors[self.tty_id].current {
            Err(())
        } else {
            self.descriptors[self.tty_id].current -= 1;
            Ok(())
        }
    }

    pub fn next_line(&mut self) -> Result<(), ()> {
        if self.descriptors[self.tty_id].current == self.descriptors[self.tty_id].end {
            Err(())
        } else {
            self.descriptors[self.tty_id].current += 1;
            Ok(())
        }
    }

    pub fn new_line(&mut self) {
        self.descriptors[self.tty_id].end =
            (self.descriptors[self.tty_id].end + 1) % HISTORY_BUFFER_HEIGHT;
        self.descriptors[self.tty_id].current = self.descriptors[self.tty_id].end;
        if (self.descriptors[self.tty_id].end + BUFFER_HEIGHT - 1) % HISTORY_BUFFER_HEIGHT
            == self.descriptors[self.tty_id].begin
        {
            self.descriptors[self.tty_id].begin =
                (self.descriptors[self.tty_id].begin + 1) % HISTORY_BUFFER_HEIGHT;
        }
        let new_line = [ScreenChar {
            ascii_character: b' ',
            color_code: DEFAULT_COLOR_CODE,
        }; BUFFER_WIDTH];
        unsafe {
            (*self.chars)[self.tty_id]
                [(self.descriptors[self.tty_id].end + BUFFER_HEIGHT - 1) % HISTORY_BUFFER_HEIGHT] =
                new_line;
        }
    }

    pub fn change_tty_id(&mut self, id: usize) -> Result<(), ()> {
        if id < NUMBER_OF_REGULAR_TTY && self.tty_id != id {
            self.tty_id = id;
            Ok(())
        } else {
            Err(())
        }
    }
}
