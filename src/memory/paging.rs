use crate::memory::{PMM, PhysAddr, PhysFrame, VirtAddr, allocate_frame};
use core::{
    arch::asm,
    ops::{BitOr, BitOrAssign},
    ptr::{addr_of, addr_of_mut},
    sync::atomic::{AtomicBool, Ordering},
};

const KERNEL_VBASE: u32 = 0xC000_0000;
const PAGE_MASK: u32 = 0xFFFF_F000;
const ENTRIES_PER_TABLE: usize = 1024;
const RECURSIVE_INDEX: usize = 1023;

// https://wiki.osdev.org/Paging
#[derive(Clone, Copy)]
pub struct PageTableFlags(u32);

#[allow(dead_code)]
impl PageTableFlags {
    pub const ACCESSED: Self = Self(1 << 5);
    pub const CACHE_DISABLE: Self = Self(1 << 4);
    pub const DIRTY: Self = Self(1 << 6);
    pub const GLOBAL: Self = Self(1 << 8);
    pub const HUGE_PAGE: Self = Self(1 << 7);
    pub const NONE: Self = Self(0);
    pub const PRESENT: Self = Self(1 << 0);
    pub const USER_ACCESSIBLE: Self = Self(1 << 2);
    pub const WRITABLE: Self = Self(1 << 1);
    pub const WRITE_THROUGH: Self = Self(1 << 3);

    pub const fn bits(self) -> u32 {
        self.0
    }

    pub const fn contains(self, flag: Self) -> bool {
        (self.0 & flag.0) == flag.0
    }

    pub const fn remove(self, flag: Self) -> Self {
        Self(self.0 & !flag.0)
    }
}

impl BitOr for PageTableFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for PageTableFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry(u32);

impl PageTableEntry {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn is_present(self) -> bool {
        (self.0 & PageTableFlags::PRESENT.bits()) != 0
    }

    pub fn frame(self) -> Option<PhysFrame> {
        if self.is_present() {
            let frame_address = self.0 & PAGE_MASK;
            Some(PhysFrame::containing_address(PhysAddr(u64::from(
                frame_address,
            ))))
        } else {
            None
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub const fn set_frame(&mut self, frame: PhysFrame, flags: PageTableFlags) {
        self.0 = (frame.start_address().0 as u32) | flags.bits();
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; ENTRIES_PER_TABLE],
}

impl PageTable {
    pub const fn empty() -> Self {
        Self {
            entries: [PageTableEntry::empty(); ENTRIES_PER_TABLE],
        }
    }

    pub fn clear(&mut self) {
        for entry in &mut self.entries {
            *entry = PageTableEntry::empty();
        }
    }
}

pub static mut KERNEL_PAGE_DIRECTORY: PageTable = PageTable::empty();

pub fn get_page_directory_address() -> u32 {
    addr_of!(KERNEL_PAGE_DIRECTORY) as u32
}

static PAGING_ACTIVE: AtomicBool = AtomicBool::new(false);

pub fn set_paging_active() {
    PAGING_ACTIVE.store(true, Ordering::SeqCst);
}

pub fn is_paging_active() -> bool {
    PAGING_ACTIVE.load(Ordering::SeqCst)
}

fn current_page_directory() -> *mut PageTable {
    if is_paging_active() {
        VirtAddr::page_directory_vaddr().0 as *mut PageTable
    } else {
        addr_of_mut!(KERNEL_PAGE_DIRECTORY)
    }
}

fn resolve_table_vaddr(virt_addr: VirtAddr, table_frame: PhysFrame) -> *mut PageTable {
    if is_paging_active() {
        virt_addr.page_table_vaddr().0 as *mut PageTable
    } else {
        table_frame.start_address().0 as *mut PageTable
    }
}

fn flush_tlb(addr: u32) {
    if is_paging_active() {
        unsafe {
            asm!(
                "invlpg [{0}]",
                in(reg) addr,
                options(nostack, preserves_flags)
            );
        }
    }
}

#[allow(clippy::similar_names)]
pub unsafe fn map_page(virt_addr: VirtAddr, phys_addr: PhysAddr, mut flags: PageTableFlags) {
    let pde_index = virt_addr.pde_index();
    let pte_index = virt_addr.pte_index();
    let is_user_space = virt_addr.0 < KERNEL_VBASE;

    if !is_user_space {
        flags = flags.remove(PageTableFlags::USER_ACCESSIBLE);
    }

    let pd_ptr = current_page_directory();
    let pd_entry = unsafe { &mut (*pd_ptr).entries[pde_index] };
    let mut is_new_table = false;
    let table_frame = pd_entry.frame().unwrap_or_else(|| {
        is_new_table = true;
        let new_table_frame = allocate_frame().expect("VMM: Out of memory during page mapping");
        let mut pde_flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        if is_user_space {
            pde_flags |= PageTableFlags::USER_ACCESSIBLE;
        }

        pd_entry.set_frame(new_table_frame, pde_flags);
        new_table_frame
    });

    let table_vaddr = resolve_table_vaddr(virt_addr, table_frame);

    unsafe {
        if is_new_table {
            (*table_vaddr).clear();
        }

        (*table_vaddr).entries[pte_index]
            .set_frame(PhysFrame::containing_address(phys_addr), flags);
    }
}

#[allow(clippy::similar_names)]
pub unsafe fn unmap_page(virt_addr: VirtAddr) {
    let pde_index = virt_addr.pde_index();
    let pd_ptr = current_page_directory();
    let pd_entry = unsafe { &mut (*pd_ptr).entries[pde_index] };
    let Some(table_frame) = pd_entry.frame() else {
        return;
    };
    let pte_index = virt_addr.pte_index();
    let table_vaddr = resolve_table_vaddr(virt_addr, table_frame);

    unsafe {
        (*table_vaddr).entries[pte_index] = PageTableEntry::empty();
    }
    flush_tlb(virt_addr.0);

    let is_table_empty = unsafe {
        !(*table_vaddr)
            .entries
            .iter()
            .any(|entry| entry.is_present())
    };

    if is_table_empty {
        *pd_entry = PageTableEntry::empty();
        flush_tlb(table_vaddr as u32);
        PMM.lock().free_frame(table_frame);
    }
}

#[allow(clippy::similar_names)]
pub fn is_page_mapped(virt_addr: VirtAddr) -> bool {
    let pde_index = virt_addr.pde_index();
    let pd_ptr = current_page_directory();
    let pd_entry = unsafe { &(*pd_ptr).entries[pde_index] };

    pd_entry.frame().is_some_and(|table_frame| {
        let pte_index = virt_addr.pte_index();
        let table_vaddr = resolve_table_vaddr(virt_addr, table_frame);

        unsafe { (*table_vaddr).entries[pte_index].is_present() }
    })
}

pub unsafe fn setup_recursive_paging() {
    let pd_phys_addr = get_page_directory_address();
    let pd_frame = PhysFrame::containing_address(PhysAddr(u64::from(pd_phys_addr)));
    let recursive_entry = unsafe { &mut KERNEL_PAGE_DIRECTORY.entries[RECURSIVE_INDEX] };

    recursive_entry.set_frame(pd_frame, PageTableFlags::PRESENT | PageTableFlags::WRITABLE);
}

pub unsafe fn load_page_directory(phys_addr: u32) {
    unsafe {
        asm!(
            "mov cr3, {0}",
            in(reg) phys_addr,
            options(nostack, preserves_flags)
        );
    }
}

pub unsafe fn enable_paging() {
    let mut cr0: u32;

    unsafe {
        asm!(
            "mov {0}, cr0",
            out(reg) cr0,
            options(nomem, nostack, preserves_flags)
        );
        cr0 |= 1 << 31;
        asm!(
            "mov cr0, {0}",
            in(reg) cr0,
            options(nostack, preserves_flags)
        );
    }
}
