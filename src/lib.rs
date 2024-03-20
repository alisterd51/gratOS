#![no_std]
#![no_main]

mod io;
mod ps2;
mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[allow(clippy::empty_loop)]
#[no_mangle]
pub extern "C" fn kmain() -> ! {
    vga_buffer::clear();

    println!("42");

    let mut keyboard = ps2::Keyboard::new();

    loop {
        keyboard.get_input();
        keyboard.to_vga_text_mode();
    }
}
