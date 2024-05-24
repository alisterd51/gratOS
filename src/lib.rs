#![no_std]
#![no_main]
#![feature(c_size_t)]

mod driver;
mod io;
mod string;

use core::panic::PanicInfo;
use driver::keyboard;
#[cfg(debug_assertions)]
use driver::tty;
use driver::vga;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    vga::text_mode::clear();

    #[cfg(debug_assertions)]
    tty::test_colors();

    println!("42");

    let mut keyboard = keyboard::ps2::Keyboard::new();

    loop {
        keyboard.get_input();
        keyboard.interpret_to_vga_text_mode();
    }
}
