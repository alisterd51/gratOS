mod halt;
mod print_kernel_stack;
mod reboot;
mod shutdown;

use super::{
    console::{self, NUMBER_OF_REGULAR_TTY},
    vga::{BUFFER_WIDTH, Line},
};
use crate::{bootprotocol, gdt, mutex::Mutex, print, println};

const PS1: &str = "> ";

#[derive(Clone, Copy, PartialEq)]
enum State {
    Uninitialized,
    Waiting,
    Ready,
}

impl State {
    const fn new() -> Self {
        Self::Uninitialized
    }
}

fn parse_command(line: &Line) -> Option<&str> {
    let line = &line[PS1.len()..];
    let end = line.iter().position(|&c| c == b'\0').unwrap_or(line.len());
    let command = &line[..end];
    core::str::from_utf8(command).ok().map(str::trim)
}

fn execute_command(line: &Line) {
    if let Some(command) = parse_command(line) {
        let mut parts = command.split_whitespace();
        let command = parts.next().unwrap_or("");
        match command {
            "help" => println!(
                "all commands:\n\thalt\n\tprint_gdt\n\tprint_multiboot\n\tprint_kernel_stack [bytes]\n\treboot\n\tshutdown\n\ttest_colors"
            ),
            "halt" => halt::halt(),
            "print_gdt" => gdt::print(),
            "print_multiboot" => bootprotocol::print(),
            "print_kernel_stack" => {
                let bytes = parts
                    .next()
                    .and_then(|arg| arg.parse::<u32>().ok())
                    .unwrap_or(0);
                print_kernel_stack::print_kernel_stack(bytes);
            }
            "reboot" => reboot::reboot(),
            "shutdown" => shutdown::qemu(),
            "test_colors" => console::test_colors(),
            "" => {}
            _ => println!("command not found"),
        }
    }
}

#[derive(Clone, Copy)]
struct Shell {
    state: State,
    line: Line,
}

impl Shell {
    const fn new() -> Self {
        Self {
            state: State::new(),
            line: [b'\0'; BUFFER_WIDTH],
        }
    }

    const fn input_line(&mut self, line: &Line) {
        self.line = *line;
    }
}

static SHELLS: [Mutex<Shell>; NUMBER_OF_REGULAR_TTY] =
    [const { Mutex::new(Shell::new()) }; NUMBER_OF_REGULAR_TTY];

fn output_prompt() {
    print!("{PS1}");
}

pub fn initialize(id: usize) {
    let mut shell = SHELLS[id].lock();
    if shell.state == State::Uninitialized {
        output_prompt();
        shell.state = State::Waiting;
    }
}

pub fn add_line_to_buf(id: usize, line: &Line) {
    let mut shell = SHELLS[id].lock();
    if shell.state == State::Waiting {
        shell.input_line(line);
        shell.state = State::Ready;
    }
}

pub fn interpret(id: usize) {
    let (state, line) = {
        let shell = SHELLS[id].lock();
        (shell.state, shell.line)
    };

    if state == State::Ready {
        execute_command(&line);
        output_prompt();
        SHELLS[id].lock().state = State::Waiting;
    }
}

pub const fn ps1() -> &'static str {
    PS1
}

// termcaps: not implemented yet
pub const fn up(_id: usize) {}

pub const fn down(_id: usize) {}

pub const fn left(_id: usize) {}

pub const fn right(_id: usize) {}
