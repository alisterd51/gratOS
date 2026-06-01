use crate::driver::{keyboard::ps2::KEYBOARD, pic::send_eoi};
use core::arch::asm;

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
            let exception_name = match frame.interrupt_number {
                0 => "Divide Error",
                1 => "Debug Exception",
                2 => "NMI Interrupt",
                3 => "Breakpoint",
                4 => "Overflow",
                5 => "BOUND Range Exceeded",
                6 => "Invalid Opcode",
                7 => "Device Not Available",
                8 => "Double Fault",
                9 => "Coprocessor Segment Overrun",
                10 => "Invalid TSS",
                11 => "Segment Not Present",
                12 => "Stack-Segment Fault",
                13 => "General Protection",
                14 => "Page Fault",
                16 => "x87 FPU Floating-Point Error",
                17 => "Alignment Check",
                18 => "Machine Check",
                19 => "SIMD Floating-Point Exception",
                20 => "Virtualization Exception",
                21 => "Control Protection Exception",
                _ => "Reserved",
            };

            let mut cr2: u32 = 0;
            if frame.interrupt_number == 14 {
                unsafe {
                    asm!("mov {}, cr2", out(reg) cr2, options(nomem, nostack, preserves_flags));
                }
            }

            if frame.interrupt_number == 14 {
                panic!(
                    "KERNEL OOPS: {} ({})\n\
                    Faulting Address (CR2): 0x{:08X}\n\
                    Error Code: 0x{:08X}\n\
                    --- Registers ---\n\
                    EAX: 0x{:08X}  EBX: 0x{:08X}  ECX: 0x{:08X}  EDX: 0x{:08X}\n\
                    ESI: 0x{:08X}  EDI: 0x{:08X}  EBP: 0x{:08X}  ESP: 0x{:08X}\n\
                    EIP: 0x{:08X}  EFLAGS: 0x{:08X}\n\
                    CS:  0x{:04X}  DS:  0x{:04X}  ES:  0x{:04X}  FS:  0x{:04X}  GS:  0x{:04X}",
                    frame.interrupt_number,
                    exception_name,
                    cr2,
                    frame.error_code,
                    frame.eax,
                    frame.ebx,
                    frame.ecx,
                    frame.edx,
                    frame.esi,
                    frame.edi,
                    frame.ebp,
                    frame.esp_dummy,
                    frame.eip,
                    frame.eflags,
                    frame.cs,
                    frame.ds,
                    frame.es,
                    frame.fs,
                    frame.gs
                );
            } else {
                panic!(
                    "KERNEL OOPS: {} ({})\n\
                    Error Code: 0x{:08X}\n\
                    --- Registers ---\n\
                    EAX: 0x{:08X}  EBX: 0x{:08X}  ECX: 0x{:08X}  EDX: 0x{:08X}\n\
                    ESI: 0x{:08X}  EDI: 0x{:08X}  EBP: 0x{:08X}  ESP: 0x{:08X}\n\
                    EIP: 0x{:08X}  EFLAGS: 0x{:08X}\n\
                    CS:  0x{:04X}  DS:  0x{:04X}  ES:  0x{:04X}  FS:  0x{:04X}  GS:  0x{:04X}",
                    frame.interrupt_number,
                    exception_name,
                    frame.error_code,
                    frame.eax,
                    frame.ebx,
                    frame.ecx,
                    frame.edx,
                    frame.esi,
                    frame.edi,
                    frame.ebp,
                    frame.esp_dummy,
                    frame.eip,
                    frame.eflags,
                    frame.cs,
                    frame.ds,
                    frame.es,
                    frame.fs,
                    frame.gs
                );
            }
        }
        32 => send_eoi(0),
        33 => {
            KEYBOARD.lock().get_input();
            send_eoi(1);
        }
        _ => {}
    }
}
