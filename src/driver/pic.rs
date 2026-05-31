// https://wiki.osdev.org/8259_PIC

use crate::io::outb;

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;
const PIC_EOI: u8 = 0x20;
const ICW1_ICW4: u8 = 0x01;
#[allow(dead_code)]
const ICW1_SINGLE: u16 = 0x02;
#[allow(dead_code)]
const ICW1_INTERVAL4: u16 = 0x04;
#[allow(dead_code)]
const ICW1_LEVEL: u16 = 0x08;
const ICW1_INIT: u8 = 0x10;
const ICW4_8086: u8 = 0x01;
#[allow(dead_code)]
const ICW4_AUTO: u16 = 0x02;
#[allow(dead_code)]
const ICW4_BUF_SLAVE: u16 = 0x08;
#[allow(dead_code)]
const ICW4_BUF_MASTER: u16 = 0x0C;
#[allow(dead_code)]
const ICW4_SFNM: u16 = 0x10;
const CASCADE_IRQ: u8 = 2;

pub fn init() {
    remap(0x20, 0x28);
}

pub fn send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            outb(PIC_EOI, PIC2_COMMAND);
        }
        outb(PIC_EOI, PIC1_COMMAND);
    }
}

fn io_wait() {
    unsafe {
        outb(0, 0x80);
    }
}

fn remap(offset1: u8, offset2: u8) {
    unsafe {
        outb(ICW1_INIT | ICW1_ICW4, PIC1_COMMAND);
        io_wait();
        outb(ICW1_INIT | ICW1_ICW4, PIC2_COMMAND);
        io_wait();
        outb(offset1, PIC1_DATA);
        io_wait();
        outb(offset2, PIC2_DATA);
        io_wait();
        outb(1 << CASCADE_IRQ, PIC1_DATA);
        io_wait();
        outb(2, PIC2_DATA);
        io_wait();

        outb(ICW4_8086, PIC1_DATA);
        io_wait();
        outb(ICW4_8086, PIC2_DATA);
        io_wait();

        outb(0, PIC1_DATA);
        outb(0, PIC2_DATA);
    }
}
