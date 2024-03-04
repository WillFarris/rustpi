use crate::memory::mmu::{
    AccessPermissions, AttributeFields, MemoryAttributes, TranslationDescription,
};
use core::{cell::UnsafeCell, str::FromStr};

use super::{PBASE_END, PBASE_START};

extern "Rust" {
    static __text_start: UnsafeCell<()>;
    static __text_end: UnsafeCell<()>;
    static __mapped_dram_start: UnsafeCell<()>;
    static __mapped_dram_end: UnsafeCell<()>;
}

pub struct KernelVirtualLayout<const NUM_SPECIAL_RANGES: usize> {
    translation_descriptions: [TranslationDescription; NUM_SPECIAL_RANGES],
}

impl<const NUM_SPECIAL_RANGES: usize> KernelVirtualLayout<{ NUM_SPECIAL_RANGES }> {
    pub fn virt_addr_properties(
        &self,
        virt_addr: usize,
    ) -> Result<(usize, AttributeFields), &'static str> {
        for desc in self.translation_descriptions.iter() {
            let physical_size = (desc.physical_end)() - (desc.physical_start)();
            if virt_addr >= (desc.virtual_start)()
                && virt_addr < (desc.virtual_start)() + physical_size
            {
                return Ok((virt_addr, desc.attributes.clone()));
            }
        }
        Err("virtual address not mapped")
    }

    pub fn layout_info(&self) -> alloc::string::String {
        let mut s = alloc::string::String::from_str("Memory layout:\n").unwrap();

        for d in self.translation_descriptions.iter() {
            let start = (d.physical_start)();
            let end = (d.physical_end)();
            let size = end - start;

            let ds = alloc::format!(
                "  {}\n    Physical range: 0x{:016X} - 0x{:016X}\n    Size: {} KiB\n    Virtual start: 0x{:X}\n    Attributes: {:?}\n    PXN: {}\n    Permisssions: {:?}\n",
                d.name,
                start,
                end,
                size / 1024,
                (d.virtual_start)(),
                d.attributes.memory_attributes,
                d.attributes.execute_never,
                d.attributes.permissions,
            );
            s.push_str(ds.as_str());
        }

        s
    }
}

const NUM_MEM_RANGES: usize = 3;
pub const KERNEL_VIRTUAL_LAYOUT: KernelVirtualLayout<NUM_MEM_RANGES> = KernelVirtualLayout {
    translation_descriptions: [
        TranslationDescription {
            name: "Kernel code (.text, .rodata)",
            physical_start: text_start,
            physical_end: text_end,
            virtual_start: text_start,
            attributes: AttributeFields {
                execute_never: false,
                permissions: AccessPermissions::ReadOnly,
                memory_attributes: MemoryAttributes::CacheableDRAM,
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
                memory_attributes: MemoryAttributes::CacheableDRAM,
            },
        },
        TranslationDescription {
            name: "MMIO (memory-mapped peripherals)",
            physical_start: mmio_start,
            physical_end: mmio_end,
            virtual_start: mmio_start,
            attributes: AttributeFields {
                execute_never: true,
                permissions: AccessPermissions::ReadWrite,
                memory_attributes: MemoryAttributes::Device,
            },
        },
    ],
};

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

pub fn virt_mem_layout() -> &'static KernelVirtualLayout<NUM_MEM_RANGES> {
    &KERNEL_VIRTUAL_LAYOUT
}
