#![no_std]
#![no_main]

mod ascii_art;
mod io;
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
    ascii_art::print_42();
    loop {}
}
