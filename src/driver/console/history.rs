use super::{DEFAULT_COLOR_CODE, HISTORY_BUFFER_HEIGHT, NUMBER_OF_REGULAR_TTY};
use crate::driver::vga::{BUFFER_HEIGHT, BUFFER_WIDTH, Screen, ScreenChar, ScreenCharLine};

static mut BUFFER: *mut [[ScreenCharLine; HISTORY_BUFFER_HEIGHT]; NUMBER_OF_REGULAR_TTY] =
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
    pub const fn new() -> Self {
        Self {
            begin: 0,
            current: 0,
            end: 0,
        }
    }
}

pub struct History {
    tty_id: usize,
    descriptors: [HistoryDescriptor; NUMBER_OF_REGULAR_TTY],
    chars: *mut [[ScreenCharLine; HISTORY_BUFFER_HEIGHT]; NUMBER_OF_REGULAR_TTY],
}

unsafe impl Send for History {}

impl History {
    pub const fn new() -> Self {
        Self {
            tty_id: 0,
            descriptors: [HistoryDescriptor::new(); NUMBER_OF_REGULAR_TTY],
            chars: unsafe { BUFFER },
        }
    }

    pub const fn set_char(&mut self, c: &ScreenChar, col: usize, row: usize) {
        if col < BUFFER_WIDTH && row < BUFFER_HEIGHT {
            unsafe {
                (*self.chars)[self.tty_id]
                    [(self.descriptors[self.tty_id].current + row) % HISTORY_BUFFER_HEIGHT][col] =
                    *c;
            }
        }
    }

    pub const fn set_line(&mut self, line: &ScreenCharLine, row: usize) {
        unsafe {
            (*self.chars)[self.tty_id]
                [(self.descriptors[self.tty_id].current + row) % HISTORY_BUFFER_HEIGHT] = *line;
        }
    }

    // TODO: remove if useless
    #[allow(dead_code)]
    pub const fn get_char(&self, x: usize, y: usize) -> Result<ScreenChar, ()> {
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
    pub const fn get_line(&self, y: usize) -> Result<ScreenCharLine, ()> {
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
        let chars_ref = unsafe { &*self.chars };
        screen.iter_mut().enumerate().for_each(|(i, s)| {
            let index = (self.descriptors[self.tty_id].current + i) % HISTORY_BUFFER_HEIGHT;
            *s = chars_ref[self.tty_id][index];
        });
        screen
    }

    pub const fn previous_line(&mut self) -> Result<(), ()> {
        if self.descriptors[self.tty_id].begin == self.descriptors[self.tty_id].current {
            Err(())
        } else {
            if self.descriptors[self.tty_id].current == 0 {
                self.descriptors[self.tty_id].current = HISTORY_BUFFER_HEIGHT - 1;
            } else {
                self.descriptors[self.tty_id].current -= 1;
            }
            Ok(())
        }
    }

    pub const fn next_line(&mut self) -> Result<(), ()> {
        if self.descriptors[self.tty_id].current == self.descriptors[self.tty_id].end {
            Err(())
        } else {
            self.descriptors[self.tty_id].current =
                (self.descriptors[self.tty_id].current + 1) % HISTORY_BUFFER_HEIGHT;
            Ok(())
        }
    }

    // TODO: remove if useless
    #[allow(dead_code)]
    pub const fn begin_line(&mut self) -> Result<(), ()> {
        let mut ok = false;

        while self.previous_line().is_ok() {
            ok = true;
        }
        if ok { Ok(()) } else { Err(()) }
    }

    pub const fn end_line(&mut self) -> Result<(), ()> {
        let mut ok = false;

        while self.next_line().is_ok() {
            ok = true;
        }
        if ok { Ok(()) } else { Err(()) }
    }

    pub const fn new_line(&mut self) {
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

    pub const fn change_tty_id(&mut self, id: usize) -> Result<(), ()> {
        if id < NUMBER_OF_REGULAR_TTY && self.tty_id != id {
            self.tty_id = id;
            Ok(())
        } else {
            Err(())
        }
    }
}
