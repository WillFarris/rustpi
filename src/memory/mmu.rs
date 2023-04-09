
mod translation_table;

pub enum AccessPermissions {
    ReadOnly,
    ReadWrite,
}

pub enum MemoryAttributes {
    CacheableDRAM,
    Device,
}

pub struct AttributeFields {
    pub execute_never: bool,
    pub permissions: AccessPermissions,
    pub memory_attributes: MemoryAttributes,
}

pub struct TranslationDescription {
    pub name: &'static str,

    pub virtual_start: fn() -> usize,
    pub physical_start: fn() -> usize,
    pub physical_end: fn() -> usize,

    pub attributes: AttributeFields,
}

#[cfg(target_arch = "aarch64")]
#[path = "../_arch/aarch64/mmu.rs"]
mod arch_mmu;
pub use arch_mmu::*;
