mod hello;

use spin::{Lazy, Mutex};

use crate::{print, println};

use super::{
    console::NUMBER_OF_REGULAR_TTY,
    vga::{Line, BUFFER_WIDTH},
};

const PS1: &str = "> ";

static SHELLS: Lazy<Mutex<[Shell; NUMBER_OF_REGULAR_TTY]>> =
    Lazy::new(|| Mutex::new([Shell::new(); NUMBER_OF_REGULAR_TTY]));
static SHELLS_STATE: Lazy<Mutex<[ShellState; NUMBER_OF_REGULAR_TTY]>> =
    Lazy::new(|| Mutex::new([ShellState::new(); NUMBER_OF_REGULAR_TTY]));

#[derive(Clone, Copy, PartialEq)]
enum ShellState {
    Uninitialized,
    Waiting,
    Ready,
}

impl ShellState {
    const fn new() -> Self {
        Self::Uninitialized
    }

    fn get(self) -> Self {
        self
    }

    fn set(&mut self, state: Self) {
        self.clone_from(&state);
    }
}

#[derive(Clone, Copy)]
struct Shell {
    command: Line,
}

impl Shell {
    const fn new() -> Self {
        Self {
            command: [b'\0'; BUFFER_WIDTH],
        }
    }

    fn input_line(&mut self, line: &Line) {
        for i in 0..BUFFER_WIDTH {
            if i < line.len() {
                self.command[i] = line[i];
            } else {
                self.command[i] = b'\0';
            }
        }
    }

    fn process(&mut self) {
        if compare_command(b"hello", &self.command) {
            hello::hello();
        } else {
            println!("command not found");
        }
    }
}

fn output_prompt() {
    print!("{PS1}");
}

// `line` contains: "{PS1}{random number of \0}{command}{random number of \0}"
fn compare_command(command: &[u8], line: &Line) -> bool {
    let mut begin = PS1.len();
    let mut end = BUFFER_WIDTH;

    for (i, c) in line.iter().enumerate().take(BUFFER_WIDTH).skip(PS1.len()) {
        if *c != b'\0' {
            begin = i;
            break;
        }
    }
    for (i, c) in line
        .iter()
        .enumerate()
        .take(BUFFER_WIDTH)
        .skip(PS1.len())
        .rev()
    {
        if *c != b'\0' {
            end = i + 1;
            break;
        }
    }
    if command.len() != (end - begin) {
        return false;
    }
    *command == line[begin..end]
}

pub fn initialize(id: usize) {
    if SHELLS_STATE.lock()[id].get() == ShellState::Uninitialized {
        output_prompt();
        SHELLS_STATE.lock()[id].set(ShellState::Waiting);
    }
}

pub fn add_line_to_buf(id: usize, line: &Line) {
    if SHELLS_STATE.lock()[id].get() == ShellState::Waiting {
        SHELLS.lock()[id].input_line(line);
        SHELLS_STATE.lock()[id].set(ShellState::Ready);
    }
}

pub fn interpret(id: usize) {
    if SHELLS_STATE.lock()[id].get() == ShellState::Ready {
        SHELLS.lock()[id].process();
        SHELLS_STATE.lock()[id].set(ShellState::Uninitialized);
    }
}

pub fn ps1() -> &'static str {
    PS1
}

// termcaps: not implemented yet
pub fn up(_id: usize) {}

pub fn down(_id: usize) {}

pub fn left(_id: usize) {}

pub fn right(_id: usize) {}
