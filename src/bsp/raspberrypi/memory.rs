use core::cell::UnsafeCell;
use crate::memory::mmu::{TranslationDescription, MemoryAccessPermissions, MemoryAttributes};

use super::{PBASE_START, PBASE_END};

extern "Rust" {
    static __text_start: UnsafeCell<()>;
    static __text_end: UnsafeCell<()>;
    static __data_start: UnsafeCell<()>;
    static __data_end: UnsafeCell<()>;
    static __stack_start: UnsafeCell<()>;
    static __stack_end: UnsafeCell<()>;
    static __heap_start: UnsafeCell<()>;
    static __heap_end: UnsafeCell<()>;
}

pub const LAYOUT: [TranslationDescription; 2] = [
    TranslationDescription {
        name: ".text",
        physical_start: text_start,
        physical_end: text_end,
        virtual_start: text_start,
        can_execute: true,
        permissions: MemoryAccessPermissions::ReadOnly,
        attributes: MemoryAttributes::CacheableDRAM,
    },
    TranslationDescription {
        name: "MMIO",
        physical_start: mmio_start,
        physical_end: mmio_end,
        virtual_start: mmio_start,
        can_execute: true,
        permissions: MemoryAccessPermissions::ReadOnly,
        attributes: MemoryAttributes::CacheableDRAM,
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