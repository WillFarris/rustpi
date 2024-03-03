mod translation_table;

pub struct TranslationGranule<const GRANULE_SIZE: usize>;

#[derive(Clone, Debug)]
pub enum AccessPermissions {
    ReadOnly,
    ReadWrite,
}

#[derive(Clone, Debug)]
pub enum MemoryAttributes {
    CacheableDRAM,
    Device,
}

#[derive(Clone)]
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

impl<const GRANULE_SIZE: usize> TranslationGranule<GRANULE_SIZE> {
    pub const SIZE: usize = Self::size_checked();
    pub const SHIFT: usize = Self::SIZE.trailing_zeros() as usize;

    const fn size_checked() -> usize {
        assert!(GRANULE_SIZE.is_power_of_two());
        GRANULE_SIZE
    }
}

#[cfg(target_arch = "aarch64")]
#[path = "../_arch/aarch64/mmu.rs"]
mod arch_mmu;
pub use arch_mmu::*;
