use core::cell::UnsafeCell;
use crate::memory::mmu::{TranslationDescription, AccessPermissions, AttributeFields, MemoryAttributes};

use super::{PBASE_START, PBASE_END};

extern "Rust" {
    static __text_start: UnsafeCell<()>;
    static __text_end: UnsafeCell<()>;
    static __mapped_dram_start: UnsafeCell<()>;
    static __mapped_dram_end: UnsafeCell<()>;
}

pub const LAYOUT: [TranslationDescription; 3] = [
    TranslationDescription {
        name: "Kernel code (.text, .rodata)",
        physical_start: text_start,
        physical_end: text_end,
        virtual_start: text_start,
        attributes: AttributeFields {
            execute_never: false,
            permissions: AccessPermissions::ReadOnly,
            memory_attributes: MemoryAttributes::CacheableDRAM
        },
        
    },
    TranslationDescription {
        name: "Mapped DRAM (.data, stack, heap)",
        physical_start: mapped_dram_start,
        physical_end: mapped_dram_end,
        virtual_start: mapped_dram_start,
        attributes: AttributeFields {
            execute_never: true,
            permissions: AccessPermissions::ReadWrite,
            memory_attributes: MemoryAttributes::CacheableDRAM
        },
    },
    TranslationDescription {
        name: "MMIO",
        physical_start: mmio_start,
        physical_end: mmio_end,
        virtual_start: mmio_start,
        attributes: AttributeFields {
            execute_never: true,
            permissions: AccessPermissions::ReadWrite,
            memory_attributes: MemoryAttributes::Device,
        },
    },
];

#[inline(always)]
fn text_start() -> usize {
    unsafe { __text_start.get() as usize }
}

#[inline(always)]
fn text_end() -> usize {
    unsafe { __text_end.get() as usize }
}

#[inline(always)]
fn mapped_dram_start() -> usize {
    unsafe { __mapped_dram_start.get() as usize }
}

#[inline(always)]
fn mapped_dram_end() -> usize {
    unsafe { __mapped_dram_end.get() as usize }
}

#[inline(always)]
fn mmio_start() -> usize {
    PBASE_START
}

#[inline(always)]
fn mmio_end() -> usize {
    PBASE_END
}

pub fn virt_mem_layout() -> &'static [TranslationDescription] {
    &LAYOUT
}