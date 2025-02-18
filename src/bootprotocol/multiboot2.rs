use crate::{bootprotocol::MemoryMapEntry, println};
use core::{
    cell::UnsafeCell,
    fmt,
    ptr::{addr_of, copy_nonoverlapping},
};

struct Multiboot2TagIter {
    current_addr: u32,
    end_addr: u32,
}

impl Iterator for Multiboot2TagIter {
    type Item = &'static Multiboot2Info;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_addr >= self.end_addr {
            return None;
        }
        let info = unsafe { &*(self.current_addr as *const Multiboot2Info) };
        if info.info_type == 0 {
            return None;
        }
        let aligned_size = (info.size + 7) & !7;
        self.current_addr += aligned_size;

        Some(info)
    }
}

impl fmt::Display for Multiboot2Tag<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommandLine(tag) => write!(f, "{tag}"),
            Self::BootLoaderName(tag) => write!(f, "{tag}"),
            Self::Modules(tag) => write!(f, "{tag}"),
            Self::BasicMemoryInfo(tag) => write!(f, "{tag}"),
            Self::BIOSBootDevice(tag) => write!(f, "{tag}"),
            Self::MemoryMap(tag) => write!(f, "{tag}"),
            Self::VBEInfo(tag) => write!(f, "{tag}"),
            Self::FramebufferInfo(tag) => write!(f, "{tag}"),
            Self::ELFSymbols(tag) => write!(f, "{tag}"),
            Self::APMTable(tag) => write!(f, "{tag}"),
            Self::EFI32bitSystemTablePointer(tag) => write!(f, "{tag}"),
            Self::EFI64bitSystemTablePointer(tag) => write!(f, "{tag}"),
            Self::SMBIOSTables(tag) => write!(f, "{tag}"),
            Self::ACPIOldRSDP(tag) => write!(f, "{tag}"),
            Self::ACPINewRSDP(tag) => write!(f, "{tag}"),
            Self::NetworkingInfo(tag) => write!(f, "{tag}"),
            Self::EFIMemoryMap(tag) => write!(f, "{tag}"),
            Self::EFIBootServicesNotTerminated(tag) => write!(f, "{tag}"),
            Self::EFI32bitImageHandlePointer(tag) => write!(f, "{tag}"),
            Self::EFI64bitImageHandlePointer(tag) => write!(f, "{tag}"),
            Self::ImageLoadBasePhysicalAddress(tag) => write!(f, "{tag}"),
            Self::Unknown(tag) => write!(f, "{tag}"),
        }
    }
}

// https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html#Boot-information-format
#[repr(C)]
struct Multiboot2BootInfo {
    total_size: u32,
    reserved: u32,
}

impl Multiboot2BootInfo {
    pub fn tags(&self) -> Multiboot2TagIter {
        let base_addr = core::ptr::from_ref::<Self>(self) as u32;

        Multiboot2TagIter {
            current_addr: base_addr + 8,
            end_addr: base_addr + self.total_size,
        }
    }
}

#[repr(C)]
struct Multiboot2Info {
    info_type: u32,
    size: u32,
}

impl Multiboot2Info {
    #[allow(clippy::cast_ptr_alignment)]
    pub const fn parse(&self) -> Multiboot2Tag<'_> {
        let ptr = core::ptr::from_ref::<Self>(self).cast::<u8>();
        unsafe {
            match self.info_type {
                1 => Multiboot2Tag::CommandLine(&*ptr.cast()),
                2 => Multiboot2Tag::BootLoaderName(&*ptr.cast()),
                3 => Multiboot2Tag::Modules(&*ptr.cast()),
                4 if self.size == 16 => Multiboot2Tag::BasicMemoryInfo(&*ptr.cast()),
                5 if self.size == 20 => Multiboot2Tag::BIOSBootDevice(&*ptr.cast()),
                6 => Multiboot2Tag::MemoryMap(&*ptr.cast()),
                7 if self.size == 784 => Multiboot2Tag::VBEInfo(&*ptr.cast()),
                8 => Multiboot2Tag::FramebufferInfo(&*ptr.cast()),
                9 => Multiboot2Tag::ELFSymbols(&*ptr.cast()),
                10 if self.size == 28 => Multiboot2Tag::APMTable(&*ptr.cast()),
                11 if self.size == 12 => Multiboot2Tag::EFI32bitSystemTablePointer(&*ptr.cast()),
                12 if self.size == 16 => Multiboot2Tag::EFI64bitSystemTablePointer(&*ptr.cast()),
                13 => Multiboot2Tag::SMBIOSTables(&*ptr.cast()),
                14 => Multiboot2Tag::ACPIOldRSDP(&*ptr.cast()),
                15 => Multiboot2Tag::ACPINewRSDP(&*ptr.cast()),
                16 => Multiboot2Tag::NetworkingInfo(&*ptr.cast()),
                17 => Multiboot2Tag::EFIMemoryMap(&*ptr.cast()),
                18 if self.size == 8 => Multiboot2Tag::EFIBootServicesNotTerminated(&*ptr.cast()),
                19 if self.size == 12 => Multiboot2Tag::EFI32bitImageHandlePointer(&*ptr.cast()),
                20 if self.size == 16 => Multiboot2Tag::EFI64bitImageHandlePointer(&*ptr.cast()),
                21 if self.size == 12 => Multiboot2Tag::ImageLoadBasePhysicalAddress(&*ptr.cast()),
                _ => Multiboot2Tag::Unknown(self),
            }
        }
    }
}

impl fmt::Display for Multiboot2Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "info: type: {}, size: {}", self.info_type, self.size)
    }
}

#[repr(C)]
struct Multiboot2BasicMemoryInfo {
    info_type: u32, // 4
    size: u32,      // 16
    mem_lower: u32,
    mem_upper: u32,
}

impl fmt::Display for Multiboot2BasicMemoryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "basic_memory_info: lower: {}, upper {}",
            self.mem_lower, self.mem_upper
        )
    }
}

#[repr(C)]
struct Multiboot2BIOSBootDevice {
    info_type: u32, // 5
    size: u32,      // 20
    biosdev: u32,
    partition: u32,
    sub_parition: u32,
}

impl fmt::Display for Multiboot2BIOSBootDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BIOS_boot_device: biosdev: {}, partition {}, sub_parition: {}",
            self.biosdev, self.partition, self.sub_parition
        )
    }
}

#[repr(C)]
struct Multiboot2BootCommandLine {
    info_type: u32, // 1
    size: u32,
}

impl Multiboot2BootCommandLine {
    pub fn as_str(&self) -> Result<&str, core::str::Utf8Error> {
        let string_length = (self.size - 8) as usize;
        unsafe {
            let string_ptr = core::ptr::from_ref::<Self>(self).cast::<u8>().add(8);
            let string_slice = core::slice::from_raw_parts(string_ptr, string_length);
            let null_pos = string_slice
                .iter()
                .position(|&b| b == 0)
                .unwrap_or(string_length);
            let valid_bytes = &string_slice[..null_pos];

            core::str::from_utf8(valid_bytes)
        }
    }
}

impl fmt::Display for Multiboot2BootCommandLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_str() {
            Ok(command) => write!(f, "boot_command_line: {command}"),
            Err(err) => write!(f, "boot_command_line: {err}"),
        }
    }
}

#[repr(C)]
struct Multiboot2Modules {
    info_type: u32, // 3
    size: u32,
    mod_start: u32,
    mod_end: u32,
}

impl Multiboot2Modules {
    pub fn as_str(&self) -> Result<&str, core::str::Utf8Error> {
        let string_length = (self.size - 16) as usize;
        unsafe {
            let string_ptr = core::ptr::from_ref::<Self>(self).cast::<u8>().add(16);
            let string_slice = core::slice::from_raw_parts(string_ptr, string_length);
            let null_pos = string_slice
                .iter()
                .position(|&b| b == 0)
                .unwrap_or(string_length);
            let valid_bytes = &string_slice[..null_pos];

            core::str::from_utf8(valid_bytes)
        }
    }
}

impl fmt::Display for Multiboot2Modules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_str() {
            Ok(modules) => write!(
                f,
                "modules: start: {}, end {}, {modules}",
                self.mod_start, self.mod_end
            ),
            Err(err) => write!(
                f,
                "modules: start: {}, end {}, {err}",
                self.mod_start, self.mod_end
            ),
        }
    }
}

#[repr(C)]
pub struct ELF32SectionHeader {
    name: u32,
    typ: u32,
    flags: u32,
    addr: u32,
    offset: u32,
    size: u32,
    link: u32,
    info: u32,
    addralign: u32,
    entsize: u32,
}

impl fmt::Display for ELF32SectionHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ELF32_section_header: name: {}, type: {}, flags: {}, addr: {}, offset: {}, size: {}, link: {}, info: {}, addralign: {}, entsize: {}",
            self.name,
            self.typ,
            self.flags,
            self.addr,
            self.offset,
            self.size,
            self.link,
            self.info,
            self.addralign,
            self.entsize,
        )
    }
}

#[repr(C)]
pub struct ELF64SectionHeader {
    name: u32,
    typ: u32,
    flags: u64,
    addr: u64,
    offset: u64,
    size: u64,
    link: u32,
    info: u32,
    addralign: u64,
    entsize: u64,
}

impl fmt::Display for ELF64SectionHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ELF64_section_header: name: {}, type: {}, flags: {}, addr: {}, offset: {}, size: {}, link: {}, info: {}, addralign: {}, entsize: {}",
            self.name,
            self.typ,
            self.flags,
            self.addr,
            self.offset,
            self.size,
            self.link,
            self.info,
            self.addralign,
            self.entsize,
        )
    }
}

pub enum ELFSectionHeaders<'a> {
    Elf32(&'a [ELF32SectionHeader]),
    Elf64(&'a [ELF64SectionHeader]),
    #[allow(dead_code)]
    Unsupported(u32),
}

#[repr(C)]
struct Multiboot2ELFSymbols {
    info_type: u32, // 9
    size: u32,
    num: u32,
    entsize: u32,
    shndx: u32,
}

impl Multiboot2ELFSymbols {
    #[allow(clippy::cast_ptr_alignment)]
    pub const fn section_headers(&self) -> ELFSectionHeaders<'_> {
        unsafe {
            let headers_ptr = core::ptr::from_ref::<Self>(self).cast::<u8>().add(20);

            match self.entsize {
                40 => {
                    let slice = core::slice::from_raw_parts(
                        headers_ptr.cast::<ELF32SectionHeader>(),
                        self.num as usize,
                    );
                    ELFSectionHeaders::Elf32(slice)
                }
                64 => {
                    let slice = core::slice::from_raw_parts(
                        headers_ptr.cast::<ELF64SectionHeader>(),
                        self.num as usize,
                    );
                    ELFSectionHeaders::Elf64(slice)
                }
                _ => ELFSectionHeaders::Unsupported(self.entsize),
            }
        }
    }
}

impl fmt::Display for Multiboot2ELFSymbols {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "ELF Symbols: size: {}, num: {}, entsize: {}, shndx: {}",
            self.size, self.num, self.entsize, self.shndx,
        )?;
        match self.section_headers() {
            ELFSectionHeaders::Elf32(section_headers) => {
                for section_header in section_headers {
                    writeln!(f, "{section_header}")?;
                }
            }
            ELFSectionHeaders::Elf64(section_headers) => {
                for section_header in section_headers {
                    writeln!(f, "{section_header}")?;
                }
            }
            ELFSectionHeaders::Unsupported(_) => {}
        }
        Ok(())
    }
}

#[repr(C)]
struct Multiboot2MemoryMap {
    info_type: u32, // 6
    size: u32,
    entry_size: u32,
    entry_version: u32,
}

#[repr(C)]
struct Multiboot2MemoryMapEntry {
    base_addr: u64,
    length: u64,
    entry_type: u32,
    reserved: u32,
}

impl fmt::Display for Multiboot2MemoryMapEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "memory_map_entry: base_addr: {}, length: {}, type: {}, reserved: {}",
            self.base_addr, self.length, self.entry_type, self.reserved
        )
    }
}

impl Multiboot2MemoryMap {
    #[allow(clippy::cast_possible_truncation, clippy::cast_ptr_alignment)]
    pub const fn entries(&self) -> &[Multiboot2MemoryMapEntry] {
        let expected_size = core::mem::size_of::<Multiboot2MemoryMapEntry>() as u32;
        if self.entry_size != expected_size {
            return &[];
        }
        let entries_total_size = self.size - 16;
        let entry_count = (entries_total_size / self.entry_size) as usize;
        unsafe {
            let entries_ptr = core::ptr::from_ref::<Self>(self).cast::<u8>().add(16);
            core::slice::from_raw_parts(entries_ptr.cast::<Multiboot2MemoryMapEntry>(), entry_count)
        }
    }
}

impl fmt::Display for Multiboot2MemoryMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "memory_map: entry_size: {}, entry_version {}:",
            self.entry_size, self.entry_version
        )?;
        for entry in self.entries() {
            writeln!(f, "{entry}")?;
        }

        Ok(())
    }
}

#[repr(C)]
struct Multiboot2BootLoaderName {
    info_type: u32, // 2
    size: u32,
}

impl Multiboot2BootLoaderName {
    pub fn as_str(&self) -> Result<&str, core::str::Utf8Error> {
        let string_length = (self.size - 8) as usize;
        unsafe {
            let string_ptr = core::ptr::from_ref::<Self>(self).cast::<u8>().add(8);
            let string_slice = core::slice::from_raw_parts(string_ptr, string_length);
            let null_pos = string_slice
                .iter()
                .position(|&b| b == 0)
                .unwrap_or(string_length);
            let valid_bytes = &string_slice[..null_pos];

            core::str::from_utf8(valid_bytes)
        }
    }
}

impl fmt::Display for Multiboot2BootLoaderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_str() {
            Ok(bootloader) => write!(f, "bootloader_name: {bootloader}"),
            Err(err) => write!(f, "bootloader_name: {err}"),
        }
    }
}

#[repr(C)]
struct Multiboot2APMTable {
    info_type: u32, // 10
    size: u32,      // 28
    version: u16,
    cseg: u16,
    offset: u32,
    cseg_16: u16,
    dseg: u16,
    flags: u16,
    cseg_len: u16,
    cseg_16_len: u16,
    dseg_len: u16,
}

impl fmt::Display for Multiboot2APMTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "APM_table: version: {}, cseg: {}, offset: {}, cseg_16: {}, dseg: {}, flags: {}, cseg_len: {}, cseg_16_len: {}, dseg_len: {}",
            self.version,
            self.cseg,
            self.offset,
            self.cseg_16,
            self.dseg,
            self.flags,
            self.cseg_len,
            self.cseg_16_len,
            self.dseg_len,
        )
    }
}

#[repr(C)]
struct Multiboot2VBEInfo {
    info_type: u32, // 7
    size: u32,      // 784
    vbe_mode: u16,
    vbe_interface_seg: u16,
    vbe_interface_off: u32,
    vbe_interface_len: u16,
    vbe_control_info: [u8; 512],
    vbe_mode_info: [u8; 256],
}

impl fmt::Display for Multiboot2VBEInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "vbe_info: mode: {}, interface_seg: {}, interface_off: {}, interface_len: {}",
            self.vbe_mode, self.vbe_interface_seg, self.vbe_interface_off, self.vbe_interface_len
        )
    }
}

#[repr(C)]
struct Multiboot2FramebufferInfo {
    info_type: u32, // 8
    size: u32,
    framebuffer_addr: u64,
    framebuffer_pitch: u32,
    framebuffer_width: u32,
    framebuffer_height: u32,
    framebuffer_bpp: u8,
    framebuffer_type: u8,
    reserved: u8,
}

#[repr(C)]
struct Multiboot2FramebufferPalette {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Multiboot2FramebufferPalette {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "palette: r {} g {} b {}",
            self.red, self.green, self.blue
        )
    }
}

#[repr(C)]
struct Multiboot2FramebufferRgbInfo {
    red_field_position: u8,
    red_mask_size: u8,
    green_field_position: u8,
    green_mask_size: u8,
    blue_field_position: u8,
    blue_mask_size: u8,
}

impl fmt::Display for Multiboot2FramebufferRgbInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rgb_info: r {} {} g {} {} b {} {}",
            self.red_field_position,
            self.red_mask_size,
            self.green_field_position,
            self.green_mask_size,
            self.blue_field_position,
            self.blue_mask_size,
        )
    }
}

enum FramebufferColorInfo<'a> {
    Indexed {
        palette: &'a [Multiboot2FramebufferPalette],
    },
    DirectRgb(&'a Multiboot2FramebufferRgbInfo),
    EgaText,
    #[allow(dead_code)]
    Unknown(u8),
}

impl Multiboot2FramebufferInfo {
    #[allow(clippy::cast_possible_truncation, clippy::cast_ptr_alignment)]
    pub const fn color_info(&self) -> FramebufferColorInfo<'_> {
        unsafe {
            let data_ptr = core::ptr::from_ref::<Self>(self).cast::<u8>().add(32);
            match self.framebuffer_type {
                0 => {
                    let num_colors = *data_ptr.cast::<u32>();
                    let palette_ptr = data_ptr.add(4).cast::<Multiboot2FramebufferPalette>();
                    let palette = core::slice::from_raw_parts(palette_ptr, num_colors as usize);

                    FramebufferColorInfo::Indexed { palette }
                }
                1 => {
                    let rgb_info = &*data_ptr.cast::<Multiboot2FramebufferRgbInfo>();

                    FramebufferColorInfo::DirectRgb(rgb_info)
                }
                2 => FramebufferColorInfo::EgaText,
                _ => FramebufferColorInfo::Unknown(self.framebuffer_type),
            }
        }
    }
}

impl fmt::Display for Multiboot2FramebufferInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "framebuffer_info: addr: {}, pitch: {}, width: {}, height: {}, bpp: {}, type: {}, reserved: {}",
            self.framebuffer_addr,
            self.framebuffer_pitch,
            self.framebuffer_width,
            self.framebuffer_height,
            self.framebuffer_bpp,
            self.framebuffer_type,
            self.reserved,
        )?;
        write!(f, "color_info: ")?;
        match self.color_info() {
            FramebufferColorInfo::Indexed { palette } => {
                for palette in palette {
                    write!(f, "{palette}")?;
                }
                Ok(())
            }
            FramebufferColorInfo::DirectRgb(rgb_info) => {
                write!(f, "{rgb_info}")
            }
            FramebufferColorInfo::EgaText => write!(f, "ega text"),
            FramebufferColorInfo::Unknown(_) => write!(f, "unknown"),
        }
    }
}

#[repr(C)]
struct Multiboot2EFI32bitSystemTablePointer {
    info_type: u32, // 11
    size: u32,      // 12
    pointer: u32,
}

impl fmt::Display for Multiboot2EFI32bitSystemTablePointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EFI_32bit_system_table_pointer: {}", self.pointer)
    }
}

#[repr(C)]
struct Multiboot2EFI64bitSystemTablePointer {
    info_type: u32, // 12
    size: u32,      // 16
    pointer: u64,
}

impl fmt::Display for Multiboot2EFI64bitSystemTablePointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EFI_64bit_system_table_pointer: {}", self.pointer)
    }
}

#[repr(C)]
struct Multiboot2SMBIOSTables {
    info_type: u32, // 13
    size: u32,
    major: u8,
    minor: u8,
    reserved: [u8; 6],
}

impl Multiboot2SMBIOSTables {
    pub const fn smbios_tables(&self) -> &[u8] {
        let data_length = (self.size - 16) as usize;
        unsafe {
            let data_ptr = core::ptr::from_ref::<Self>(self).cast::<u8>().add(16);
            core::slice::from_raw_parts(data_ptr, data_length)
        }
    }
}

impl fmt::Display for Multiboot2SMBIOSTables {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let smbios_tables = self.smbios_tables();
        let signature = if smbios_tables.len() >= 4
            && let Ok(signature) = core::str::from_utf8(&smbios_tables[0..4])
        {
            signature
        } else {
            ""
        };

        write!(
            f,
            "SMBIOS_tables: version: {}.{}, signature: {signature}",
            self.major, self.minor
        )
    }
}

#[repr(C)]
pub struct AcpiRsdpV1 {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
}

impl AcpiRsdpV1 {
    pub const fn signature_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.signature)
    }

    pub const fn oem_id_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.oem_id)
    }
}

impl fmt::Display for AcpiRsdpV1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let signature = self.signature_str().unwrap_or_default();
        let oem_id = self.oem_id_str().unwrap_or_default();

        write!(f, "signature: {signature}, oem_id: {oem_id}")
    }
}

#[repr(C)]
pub struct AcpiRsdpV2 {
    pub v1: AcpiRsdpV1,
    pub length: u32,
    pub xsdt_address: u64,
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}

#[repr(C)]
struct Multiboot2ACPIOldRSDP {
    info_type: u32, // 14
    size: u32,
}

impl Multiboot2ACPIOldRSDP {
    #[allow(clippy::cast_ptr_alignment)]
    pub const fn rsdp(&self) -> Option<&AcpiRsdpV1> {
        if self.size < 28 {
            return None;
        }
        unsafe {
            let rsdp_ptr = core::ptr::from_ref::<Self>(self).cast::<u8>().add(8);
            Some(&*rsdp_ptr.cast::<AcpiRsdpV1>())
        }
    }
}

impl fmt::Display for Multiboot2ACPIOldRSDP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ACPI old RSDP: ")?;
        match self.rsdp() {
            Some(rsdp) => write!(f, "rsdp: {rsdp}"),
            None => write!(f, "no RSDP"),
        }
    }
}

#[repr(C)]
struct Multiboot2ACPINewRSDP {
    info_type: u32, // 15
    size: u32,
}

impl Multiboot2ACPINewRSDP {
    #[allow(clippy::cast_ptr_alignment)]
    pub const fn rsdp(&self) -> Option<&AcpiRsdpV2> {
        if self.size < 44 {
            return None;
        }
        unsafe {
            let rsdp_ptr = core::ptr::from_ref::<Self>(self).cast::<u8>().add(8);
            Some(&*rsdp_ptr.cast::<AcpiRsdpV2>())
        }
    }
}

impl fmt::Display for Multiboot2ACPINewRSDP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ACPI old RSDP: ")?;
        match self.rsdp() {
            Some(rsdp) => write!(f, "rsdp: {}", rsdp.v1),
            None => write!(f, "no RSDP"),
        }
    }
}

#[repr(C)]
struct Multiboot2NetworkingInfo {
    info_type: u32, // 16
    size: u32,
}

impl Multiboot2NetworkingInfo {
    pub const fn dhcp_ack(&self) -> &[u8] {
        let data_length = (self.size - 8) as usize;
        unsafe {
            let data_ptr = core::ptr::from_ref::<Self>(self).cast::<u8>().add(8);
            core::slice::from_raw_parts(data_ptr, data_length)
        }
    }
}

impl fmt::Display for Multiboot2NetworkingInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "networking_info: len: {}", self.dhcp_ack().len())
    }
}

#[repr(C)]
struct Multiboot2EFIMemoryMap {
    info_type: u32, // 17
    size: u32,
    descriptor_size: u32,
    descriptor_version: u32,
}

impl Multiboot2EFIMemoryMap {
    #[allow(dead_code)]
    pub const fn efi_memory_map(&self) -> &[u8] {
        let data_length = (self.size - 16) as usize;
        unsafe {
            let data_ptr = core::ptr::from_ref::<Self>(self).cast::<u8>().add(16);
            core::slice::from_raw_parts(data_ptr, data_length)
        }
    }
}

impl fmt::Display for Multiboot2EFIMemoryMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EFI_memory_map: escriptor: size: {}, version: {}",
            self.descriptor_size, self.descriptor_version
        )
    }
}

#[repr(C)]
struct Multiboot2EFIBootServicesNotTerminated {
    info_type: u32, // 18
    size: u32,      // 8
}

impl fmt::Display for Multiboot2EFIBootServicesNotTerminated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EFI_boot_services_not_terminated")
    }
}

#[repr(C)]
struct Multiboot2EFI32bitImageHandlePointer {
    info_type: u32, // 19
    size: u32,      // 12
    pointer: u32,
}

impl fmt::Display for Multiboot2EFI32bitImageHandlePointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EFI_32bit_image_handle_pointer: {}", self.pointer)
    }
}

#[repr(C)]
struct Multiboot2EFI64bitImageHandlePointer {
    info_type: u32, // 20
    size: u32,      // 16
    pointer: u64,
}

impl fmt::Display for Multiboot2EFI64bitImageHandlePointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EFI_64bit_image_handle_pointer: {}", self.pointer)
    }
}

#[repr(C)]
struct Multiboot2ImageLoadBasePhysicalAddress {
    info_type: u32, // 21
    size: u32,      // 12
    load_base_addr: u32,
}

impl fmt::Display for Multiboot2ImageLoadBasePhysicalAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "image_load_base_physical_address: {}",
            self.load_base_addr
        )
    }
}

enum Multiboot2Tag<'a> {
    CommandLine(&'a Multiboot2BootCommandLine),     // 1
    BootLoaderName(&'a Multiboot2BootLoaderName),   // 2
    Modules(&'a Multiboot2Modules),                 // 3
    BasicMemoryInfo(&'a Multiboot2BasicMemoryInfo), // 4
    BIOSBootDevice(&'a Multiboot2BIOSBootDevice),   // 5
    MemoryMap(&'a Multiboot2MemoryMap),             // 6
    VBEInfo(&'a Multiboot2VBEInfo),                 // 7
    FramebufferInfo(&'a Multiboot2FramebufferInfo), // 8
    ELFSymbols(&'a Multiboot2ELFSymbols),           // 9
    APMTable(&'a Multiboot2APMTable),               // 10
    EFI32bitSystemTablePointer(&'a Multiboot2EFI32bitSystemTablePointer), // 11
    EFI64bitSystemTablePointer(&'a Multiboot2EFI64bitSystemTablePointer), // 12
    SMBIOSTables(&'a Multiboot2SMBIOSTables),       // 13
    ACPIOldRSDP(&'a Multiboot2ACPIOldRSDP),         // 14
    ACPINewRSDP(&'a Multiboot2ACPINewRSDP),         // 15
    NetworkingInfo(&'a Multiboot2NetworkingInfo),   // 16
    EFIMemoryMap(&'a Multiboot2EFIMemoryMap),       // 17
    EFIBootServicesNotTerminated(&'a Multiboot2EFIBootServicesNotTerminated), // 18
    EFI32bitImageHandlePointer(&'a Multiboot2EFI32bitImageHandlePointer), // 19
    EFI64bitImageHandlePointer(&'a Multiboot2EFI64bitImageHandlePointer), // 20
    ImageLoadBasePhysicalAddress(&'a Multiboot2ImageLoadBasePhysicalAddress), // 21
    Unknown(&'a Multiboot2Info),                    // other
}

const MULTIBOOT2_CACHE_SIZE: usize = 8192;
const MAX_MEMORY_ENTRIES: usize = 64;

#[repr(C, align(8))]
struct Multiboot2Buffer([u8; MULTIBOOT2_CACHE_SIZE]);

struct BootCache {
    is_ready: bool,
    size: usize,
    blob: Multiboot2Buffer,
    memory_map: [MemoryMapEntry; MAX_MEMORY_ENTRIES],
    memory_map_count: usize,
}

impl BootCache {
    const fn empty() -> Self {
        Self {
            is_ready: false,
            size: 0,
            blob: Multiboot2Buffer([0; MULTIBOOT2_CACHE_SIZE]),
            memory_map: [MemoryMapEntry::empty(); MAX_MEMORY_ENTRIES],
            memory_map_count: 0,
        }
    }
}

struct GlobalCache(UnsafeCell<BootCache>);

unsafe impl Sync for GlobalCache {}

static CACHE: GlobalCache = GlobalCache(UnsafeCell::new(BootCache::empty()));

pub fn init(info_addr: u32) {
    let boot_info = unsafe { &*(info_addr as *const Multiboot2BootInfo) };
    let size = boot_info.total_size as usize;
    let size = size.min(MULTIBOOT2_CACHE_SIZE);
    let cache = unsafe { &mut *CACHE.0.get() };

    unsafe {
        copy_nonoverlapping(info_addr as *const u8, cache.blob.0.as_mut_ptr(), size);
    }
    cache.size = size;
    cache.is_ready = true;

    let boot_info = unsafe { &*addr_of!(cache.blob).cast::<Multiboot2BootInfo>() };

    cache.memory_map_count = 0;
    for info in boot_info.tags() {
        if let Multiboot2Tag::MemoryMap(memory_map) = info.parse() {
            for entry in memory_map.entries() {
                if cache.memory_map_count >= MAX_MEMORY_ENTRIES {
                    break;
                }

                cache.memory_map[cache.memory_map_count] = MemoryMapEntry {
                    base_addr: entry.base_addr,
                    length: entry.length,
                    entry_type: entry.entry_type,
                };
                cache.memory_map_count += 1;
            }
        }
    }
}

pub fn get_memory_map() -> &'static [MemoryMapEntry] {
    let cache = unsafe { &*CACHE.0.get() };
    &cache.memory_map[..cache.memory_map_count]
}

pub fn print() {
    let cache = unsafe { &*CACHE.0.get() };

    if cache.is_ready {
        let boot_info = unsafe { &*addr_of!(cache.blob).cast::<Multiboot2BootInfo>() };

        println!(
            "total_size: {}, reserved: {}",
            boot_info.total_size, boot_info.reserved
        );
        println!();
        for info in boot_info.tags() {
            let tag = info.parse();
            println!("{tag}");
        }
    }
}
