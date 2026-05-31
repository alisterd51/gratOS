#![no_std]
#![no_main]

extern crate alloc;

mod bootprotocol;
mod driver;
mod gdt;
mod idt;
mod interrupts;
mod io;
mod memory;
mod mutex;
mod power;

use crate::{
    driver::{
        console::{self, RESET},
        keyboard::ps2::KEYBOARD,
        pic, shell,
    },
    power::halt,
};
use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{RESET}\n{info}");
    halt::halt();
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain(magic: u32, info_addr: u32) -> ! {
    console::clear();
    gdt::init();
    idt::init();
    bootprotocol::init(magic, info_addr);
    memory::init();
    pic::init();

    loop {
        shell::initialize(console::get_tty_id());
        unsafe { asm!("cli", options(nomem, nostack, preserves_flags)) };
        KEYBOARD.lock().interpret_to_vga_text_mode();
        shell::interpret(console::get_tty_id());
        unsafe { asm!("sti", "hlt", options(nomem, nostack, preserves_flags)) };
    }
}

#[cfg(feature = "multiboot")]
global_asm!(include_str!("multiboot.s"), options(att_syntax));
#[cfg(feature = "multiboot2")]
global_asm!(include_str!("multiboot2.s"), options(att_syntax));
global_asm!(include_str!("start.s"), options(att_syntax));
global_asm!(include_str!("interrupts.s"), options(att_syntax));
