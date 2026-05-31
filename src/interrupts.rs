use crate::{
    driver::{
        console::{FG_RED, FG_RESET, RESET},
        keyboard::ps2::KEYBOARD,
        pic::send_eoi,
    },
    power::halt::halt,
    print, println,
};

#[repr(C)]
pub struct InterruptFrame {
    pub gs: u32,
    pub fs: u32,
    pub es: u32,
    pub ds: u32,
    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub esp_dummy: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,
    pub interrupt_number: u32,
    pub error_code: u32,
    pub eip: u32,
    pub cs: u32,
    pub eflags: u32,
}

#[unsafe(no_mangle)]
pub extern "C" fn interrupt_dispatcher(frame: &mut InterruptFrame) {
    match frame.interrupt_number {
        0..32 => {
            println!("\n{RESET}{FG_RED}KERNEL PANIC");
            println!("interrupt number: {}", frame.interrupt_number);
            println!("EIP: 0x{:08X}", frame.eip);
            println!("error code: 0x{:08X}", frame.error_code);
            print!("{FG_RESET}");
            halt();
        }
        32 => send_eoi(0),
        33 => {
            KEYBOARD.lock().get_input();
            send_eoi(1);
        }
        _ => {}
    }
}
