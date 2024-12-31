#![no_std]
#![no_main]

mod driver;
mod gdt;
mod io;

use core::arch::global_asm;
use core::panic::PanicInfo;
use driver::console;
use driver::keyboard;
use driver::shell;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    gdt::init();

    console::clear();

    #[cfg(debug_assertions)]
    console::test_colors();

    println!("{}42{}", console::FG_GREEN, console::FG_RESET);

    let mut keyboard = keyboard::ps2::Keyboard::new();

    loop {
        shell::initialize(console::get_tty_id());
        keyboard.get_input();
        keyboard.interpret_to_vga_text_mode();
        shell::interpret(console::get_tty_id());
    }
}

global_asm!(include_str!("start.s"), options(att_syntax));
