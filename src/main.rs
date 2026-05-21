#![no_std]
#![no_main]

mod bootprotocol;
mod driver;
mod gdt;
mod io;
mod mutex;

use core::{arch::global_asm, panic::PanicInfo};
use driver::{console, keyboard, shell};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain(magic: u32, info_addr: u32) -> ! {
    bootprotocol::init(magic, info_addr);
    gdt::init();
    console::clear();

    println!("{}42{}", console::FG_GREEN, console::FG_RESET);

    let mut keyboard = keyboard::ps2::Keyboard::new();

    loop {
        shell::initialize(console::get_tty_id());
        keyboard.get_input();
        keyboard.interpret_to_vga_text_mode();
        shell::interpret(console::get_tty_id());
    }
}

#[cfg(feature = "multiboot")]
global_asm!(include_str!("multiboot.s"), options(att_syntax));
#[cfg(feature = "multiboot2")]
global_asm!(include_str!("multiboot2.s"), options(att_syntax));
global_asm!(include_str!("start.s"), options(att_syntax));
