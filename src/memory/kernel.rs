use crate::memory::address::PhysAddr;
use core::ptr::addr_of;

unsafe extern "C" {
    static _kernel_start: u8;
    static _kernel_end: u8;
}

pub fn start() -> PhysAddr {
    PhysAddr(addr_of!(_kernel_start) as u64)
}

pub fn end() -> PhysAddr {
    PhysAddr(addr_of!(_kernel_end) as u64)
}

pub fn size() -> u64 {
    end().0 - start().0
}
