use stivale_boot::v2::{StivaleMemoryMapEntryType as MType, StivaleMemoryMapEntry as MEntry};

#[repr(u32)]
#[derive(Copy, Clone, PartialEq)]
pub enum MemEType {
    Usable = 1,
    Reserved = 2,
    AcpiReclaimable = 3,
    AcpiNvs = 4,
    BadMemory = 5,
    BootloaderReclaimable = 0x1000,
    Kernel = 0x1001,
    Framebuffer = 0x1002,
}
impl MemEType {
    pub fn from_stivale_memtype(m: MType) -> Self {
        match m {
            MType::Usable => Self::Usable,
            MType::Reserved => Self::Reserved,
            MType::AcpiReclaimable => Self::AcpiReclaimable,
            MType::AcpiNvs => Self::AcpiNvs,
            MType::BadMemory => Self::BadMemory,
            MType::BootloaderReclaimable => Self::BootloaderReclaimable,
            MType::Kernel => Self::Kernel,
            MType::Framebuffer => Self::Framebuffer,
        }
    }
}
#[derive(Copy, Clone)]
pub struct MemEntry {
    start: u64,
    size: u64,
    etype: MemEType,
}
impl MemEntry {
    pub const fn default() -> Self {
        Self {
            start: 0,
            size: 0,
            etype: MemEType::BadMemory,
        }
    }
    pub fn from_stivale_mem(m: MEntry) -> Self {
        Self {
            start: m.base,
            size: m.length,
            etype: MemEType::from_stivale_memtype(m.entry_type),
        }
    }
}

pub const ENTRIES_COUNT: usize = 32;
pub const PAGE_SIZE: u64 = 4096;
mod pmm;
pub use pmm::{init_memory_map, free_page, init_pmm, alloc_page};
