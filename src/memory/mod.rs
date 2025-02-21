use core::{
    arch::asm,
    ptr::{addr_of, addr_of_mut},
};

// Page Directory Entry (PDE)
// Page Table Entry (PTE)

const P_P: u32 = 1 << 0; // Present
const P_RW: u32 = 1 << 1; // Read/Write
const P_US: u32 = 1 << 2; // User/Supervisor
const P_PWT: u32 = 1 << 3; // Page Write-Through
const P_PCD: u32 = 1 << 4; // Page Cache Disable
const P_A: u32 = 1 << 5; // Accessed

const PTE_D: u32 = 1 << 6; // Dirty
const PTE_PAT: u32 = 1 << 7; // Page Attribute Table
const PTE_G: u32 = 1 << 8; // Global

#[repr(align(4096))]
struct PageDirectory([u32; 1024]);

impl PageDirectory {
    pub const fn new() -> Self {
        Self([0; 1024])
    }
}

#[repr(align(4096))]
struct PageTable([u32; 1024]);

impl PageTable {
    pub const fn new() -> Self {
        Self([0; 1024])
    }
}

static mut PAGE_DIRECTORY: PageDirectory = PageDirectory::new();
static mut FIRST_PAGE_TABLE: PageTable = PageTable::new();

unsafe fn enable_paging(page_directory: *mut PageDirectory) {
    unsafe {
        asm!(
            "mov cr3, {page_directory}",
            "mov {tmp}, cr0",
            "or {tmp}, 0x80000001",
            "mov cr0, {tmp}",
            "mov {tmp}, cr4",
            "or {tmp}, 0x00000010",
            "mov cr4, {tmp}",
            page_directory = in(reg) page_directory,
            tmp = lateout(reg) _,
            options(nostack, preserves_flags)
        );
    }
}

pub fn init() {
    let mut i: u32 = 0;
    while i < 1024 {
        unsafe {
            PAGE_DIRECTORY.0[i as usize] = P_RW;
        }
        i += 1;
    }
    let mut i: u32 = 0;
    while i < 1024 {
        unsafe {
            FIRST_PAGE_TABLE.0[i as usize] = (i * 0x1000) | P_RW | P_P;
        }
        i += 1;
    }
    unsafe {
        PAGE_DIRECTORY.0[0] = (addr_of!(FIRST_PAGE_TABLE) as u32) | P_RW | P_P;
        enable_paging(addr_of_mut!(PAGE_DIRECTORY));
    }
}

// il faut creer un system d'alocation de page: <https://wiki.osdev.org/Page_Frame_Allocation>

// faire un allocateur:
// https://wiki.osdev.org/Memory_Allocation

// Physical Memory Manager:

// 1ere etape:
// Page Frame Allocator:
// alloc et free pages
// - bitmap
// - buddy allocator
// 1 Page Frame descriptor par Page Frame

// 2eme etape
// Paging:
// interface map page virtuel sur page physique (page table, page directory), changer le maping
// changement de page directory

// 3eme etape:
// Virtual Space Allocator
// alloc et free de l'espace virtuel
//

// alloc objet:
// - SLAB Allocator
//   - alloc des nouveau cache
//   - un cache alloue staticement pour allouer les autres cache
// kmalloc memoir contigu, pas arrondi (SLAB Allocator)
//  - utilise les cache du SLAB Allocator
// vmalloc memoir non physiquement contigu, arroundi a la page superieur
//  - virtual space allocator alloc
//  - alloc les pages
//  - map phy/virt
