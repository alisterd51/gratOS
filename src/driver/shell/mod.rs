mod debug;
mod hexdump;

use super::{
    console::{self, NUMBER_OF_REGULAR_TTY},
    vga::{BUFFER_WIDTH, Line},
};
use crate::{
    bootprotocol,
    driver::keyboard::ps2::KEYBOARD,
    gdt, memory,
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
                \ttest_invalid_opcode\n\
                \tazerty\n\
                \tqwerty"
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
            "azerty" => {
                KEYBOARD.lock().set_azerty();
                println!("Keyboard layout set to AZERTY (fr)");
            }
            "qwerty" => {
                KEYBOARD.lock().set_qwerty();
                println!("Keyboard layout set to US QWERTY");
            }
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
