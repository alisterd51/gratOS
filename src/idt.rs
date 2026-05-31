// https://wiki.osdev.org/Interrupt_Descriptor_Table

use crate::gdt::KERNEL_CODE_SEGMENT_SELECTOR;
use core::arch::asm;

const IDT_ENTRIES: usize = 256;

const GATE_PRESENT: u8 = 1 << 7;
const GATE_DPL_RING0: u8 = 0 << 5;
#[allow(dead_code)]
const GATE_DPL_RING3: u8 = 3 << 5;
#[allow(dead_code)]
const GATE_TYPE_TASK: u8 = 0x5;
#[allow(dead_code)]
const GATE_TYPE_INTERRUPT_16: u8 = 0x6;
#[allow(dead_code)]
const GATE_TYPE_TRAP_16: u8 = 0x7;
const GATE_TYPE_INTERRUPT_32: u8 = 0xE;
#[allow(dead_code)]
const GATE_TYPE_TRAP_32: u8 = 0xF;

const INTERRUPT_GATE_FLAGS: u8 = GATE_PRESENT | GATE_DPL_RING0 | GATE_TYPE_INTERRUPT_32;

unsafe extern "C" {
    fn isr0();
    fn isr1();
    fn isr2();
    fn isr3();
    fn isr4();
    fn isr5();
    fn isr6();
    fn isr7();
    fn isr8();
    fn isr9();
    fn isr10();
    fn isr11();
    fn isr12();
    fn isr13();
    fn isr14();
    fn isr15();
    fn isr16();
    fn isr17();
    fn isr18();
    fn isr19();
    fn isr20();
    fn isr21();
    fn isr22();
    fn isr23();
    fn isr24();
    fn isr25();
    fn isr26();
    fn isr27();
    fn isr28();
    fn isr29();
    fn isr30();
    fn isr31();
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct GateDescriptor {
    offset_low: u16,
    segment_selector: u16,
    reserved: u8,
    flags: u8,
    offset_high: u16,
}

impl GateDescriptor {
    pub const fn new(offset: u32, segment_selector: u16, flags: u8) -> Self {
        Self {
            offset_low: (offset & 0xFFFF) as u16,
            segment_selector,
            reserved: 0,
            flags,
            offset_high: ((offset >> 16) & 0xFFFF) as u16,
        }
    }

    pub const fn null() -> Self {
        Self::new(0, 0, 0)
    }
}

#[repr(C, packed)]
struct IdtDescriptor {
    limit: u16,
    base: u32,
}

#[repr(C, align(16))]
pub struct Idt([GateDescriptor; IDT_ENTRIES]);

impl Idt {
    pub const fn new() -> Self {
        Self([GateDescriptor::null(); IDT_ENTRIES])
    }

    #[allow(dead_code)]
    pub const fn set_handler(&mut self, interrupt_number: u8, handler_address: u32) {
        self.0[interrupt_number as usize] = GateDescriptor::new(
            handler_address,
            KERNEL_CODE_SEGMENT_SELECTOR,
            INTERRUPT_GATE_FLAGS,
        );
    }
}

static mut IDT: Idt = Idt::new();

#[allow(clippy::cast_possible_truncation)]
pub fn init() {
    let isrs: [unsafe extern "C" fn(); 32] = [
        isr0, isr1, isr2, isr3, isr4, isr5, isr6, isr7, isr8, isr9, isr10, isr11, isr12, isr13,
        isr14, isr15, isr16, isr17, isr18, isr19, isr20, isr21, isr22, isr23, isr24, isr25, isr26,
        isr27, isr28, isr29, isr30, isr31,
    ];

    unsafe {
        let idt_ptr = &raw mut IDT;

        for (i, isr) in isrs.iter().enumerate() {
            (*idt_ptr).set_handler(i as u8, *isr as usize as u32);
        }
    }

    let idtr = IdtDescriptor {
        limit: (core::mem::size_of::<Idt>() - 1) as u16,
        base: (&raw const IDT) as u32,
    };

    unsafe {
        asm!(
            "lidt [{}]",
            in(reg) &raw const idtr,
            options(nostack, preserves_flags)
        );
    }
}
