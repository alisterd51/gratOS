mod halt;
mod print_gdt;
mod print_kernel_stack;
mod reboot;
mod shutdown;

use super::{
    console::{self, NUMBER_OF_REGULAR_TTY},
    vga::{BUFFER_WIDTH, Line},
};
use crate::{mutex::Mutex, print, println};

const PS1: &str = "> ";

static SHELLS: Mutex<[Shell; NUMBER_OF_REGULAR_TTY]> =
    Mutex::new([Shell::new(); NUMBER_OF_REGULAR_TTY]);
static SHELLS_STATE: Mutex<[ShellState; NUMBER_OF_REGULAR_TTY]> =
    Mutex::new([ShellState::new(); NUMBER_OF_REGULAR_TTY]);

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

    const fn get(self) -> Self {
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
        self.command.iter_mut().enumerate().for_each(|(i, cmd)| {
            if i < line.len() {
                *cmd = line[i];
            } else {
                *cmd = b'\0';
            }
        });
    }

    fn process(&self) {
        if compare_command(b"help", &self.command) {
            println!(
                "all commands:\n\thalt\n\tprint_gdt\n\tprint_kernel_stack\n\tprint_kernel_stack_test\n\treboot\n\tshutdown\n\ttest_colors"
            );
        } else if compare_command(b"halt", &self.command) {
            halt::halt();
        } else if compare_command(b"print_gdt", &self.command) {
            print_gdt::print_gdt();
        } else if compare_command(b"print_kernel_stack", &self.command) {
            print_kernel_stack::print_kernel_stack(0);
        } else if compare_command(b"print_kernel_stack_test", &self.command) {
            print_kernel_stack::test();
        } else if compare_command(b"reboot", &self.command) {
            reboot::reboot();
        } else if compare_command(b"shutdown", &self.command) {
            shutdown::qemu();
        } else if compare_command(b"test_colors", &self.command) {
            console::test_colors();
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

    if let Some((i, _)) = line
        .iter()
        .enumerate()
        .take(BUFFER_WIDTH)
        .skip(PS1.len())
        .find(|&(_, &c)| c != b'\0')
    {
        begin = i;
    }
    if let Some((i, _)) = line
        .iter()
        .enumerate()
        .take(BUFFER_WIDTH)
        .skip(PS1.len())
        .rev()
        .find(|&(_, &c)| c != b'\0')
    {
        end = i + 1;
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

pub const fn ps1() -> &'static str {
    PS1
}

// termcaps: not implemented yet
pub const fn up(_id: usize) {}

pub const fn down(_id: usize) {}

pub const fn left(_id: usize) {}

pub const fn right(_id: usize) {}
