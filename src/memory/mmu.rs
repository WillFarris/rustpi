
pub enum MemoryAccessPermissions {
    ReadOnly,
    ReadWrite,
}

pub enum MemoryAttributes {
    CacheableDRAM,
    Device,
}

pub struct TranslationDescription {
    pub name: &'static str,

    pub virtual_start: fn() -> usize,
    pub physical_start: fn() -> usize,
    pub physical_end: fn() -> usize,
    
    pub can_execute: bool,
    pub permissions: MemoryAccessPermissions,
    pub attributes: MemoryAttributes,
}

#[cfg(target_arch = "aarch64")]
#[path = "../_arch/aarch64/mmu.rs"]
mod arch_mmu;
pub use arch_mmu::*;
