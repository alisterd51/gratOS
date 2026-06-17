mod debug;
mod hexdump;

use super::{
    console::{self, NUMBER_OF_REGULAR_TTY},
    vga::{COMMAND_LINE_LENGTH, CommandLine},
};
use crate::{
    bootprotocol, gdt, memory,
    mutex::Mutex,
    power::{halt, reboot, shutdown},
    print, println,
};
use core::{arch::asm, ptr::read_volatile};

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

fn parse_command(line: &CommandLine) -> Option<&str> {
    let end = line.iter().position(|&c| c == b'\0').unwrap_or(line.len());
    let command = &line[..end];
    core::str::from_utf8(command).ok().map(str::trim)
}

fn execute_command(line: &CommandLine) {
    if let Some(command) = parse_command(line) {
        let mut parts = command.split_whitespace();
        let command = parts.next().unwrap_or("");
        let arg = parts.next();

        match command {
            "help" => println!(
                "all commands:\n\
                \thalt\n\
                \tpanic\n\
                \tprint_gdt\n\
                \tprint_memory\n\
                \tprint_multiboot\n\
                \talloc_heap [size]\n\
                \tfree_heap\n\
                \talloc_dma [size]\n\
                \tfree_dma\n\
                \tdump_stack [bytes]\n\
                \tdump_heap [bytes]\n\
                \tdump_dma [bytes]\n\
                \tdump_vga [lines]\n\
                \treboot\n\
                \tshutdown\n\
                \ttest_colors\n\
                \ttest_page_fault\n\
                \ttest_invalid_opcode"
            ),
            "halt" => halt::halt(),
            "panic" => panic!(),
            "print_gdt" => gdt::print(),
            "print_memory" => memory::print(),
            "print_multiboot" => bootprotocol::print(),
            "alloc_heap" => {
                let size = arg.and_then(|a| a.parse::<usize>().ok()).unwrap_or(64);

                debug::alloc_heap(size);
            }
            "free_heap" => {
                debug::free_heap();
            }
            "alloc_dma" => {
                let size = arg.and_then(|a| a.parse::<usize>().ok()).unwrap_or(4096);

                debug::alloc_dma(size);
            }
            "free_dma" => {
                debug::free_dma();
            }
            "dump_stack" => {
                let bytes = arg.and_then(|a| a.parse::<usize>().ok()).unwrap_or(128);

                debug::print_stack(bytes);
            }
            "dump_heap" => {
                let bytes = arg.and_then(|a| a.parse::<usize>().ok()).unwrap_or(128);

                debug::print_heap(bytes);
            }
            "dump_dma" => {
                let bytes = arg.and_then(|a| a.parse::<usize>().ok()).unwrap_or(128);

                debug::print_dma(bytes);
            }
            "dump_vga" => {
                let lines = arg.and_then(|a| a.parse::<usize>().ok()).unwrap_or(25);

                debug::print_vga(lines);
            }
            "reboot" => reboot::reboot(),
            "shutdown" => shutdown::qemu(),
            "test_colors" => console::test_colors(),
            "test_page_fault" => unsafe {
                let _ = read_volatile(0xDEAD_BEEF as *const u32);
            },
            "test_invalid_opcode" => unsafe {
                asm!("ud2", options(nomem, nostack));
            },
            "" => {}
            _ => println!("command not found"),
        }
    }
}

#[derive(Clone, Copy)]
struct Shell {
    state: State,
    line: CommandLine,
    len: usize,
}

impl Shell {
    const fn new() -> Self {
        Self {
            state: State::new(),
            line: [b'\0'; COMMAND_LINE_LENGTH],
            len: 0,
        }
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

pub fn push_char(id: usize, byte: u8) {
    let mut shell = SHELLS[id].lock();
    if shell.state == State::Waiting && shell.len < COMMAND_LINE_LENGTH {
        let len = shell.len;
        shell.line[len] = byte;
        shell.len = len + 1;
    }
}

pub fn pop_char(id: usize) -> bool {
    let mut shell = SHELLS[id].lock();
    if shell.state == State::Waiting && shell.len > 0 {
        shell.len -= 1;
        let len = shell.len;
        shell.line[len] = b'\0';
        true
    } else {
        false
    }
}

pub fn submit(id: usize) {
    let mut shell = SHELLS[id].lock();
    if shell.state == State::Waiting {
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
        {
            let mut shell = SHELLS[id].lock();
            shell.line = [b'\0'; COMMAND_LINE_LENGTH];
            shell.len = 0;
            shell.state = State::Waiting;
        }
        output_prompt();
    }
}

// termcaps: not implemented yet
pub const fn up(_id: usize) {}

pub const fn down(_id: usize) {}

pub const fn left(_id: usize) {}

pub const fn right(_id: usize) {}
