use core::convert;
use tock_registers::register_bitfields;

use crate::memory::mmu::{AttributeFields, TranslationDescription};

// A table descriptor, as per ARMv8-A Architecture Reference Manual Figure D5-15.
register_bitfields! {u64,
    STAGE1_TABLE_DESCRIPTOR [
        /// Physical address of the next descriptor.
        NEXT_LEVEL_TABLE_ADDR_64KiB OFFSET(16) NUMBITS(32) [], // [47:16]

        TYPE  OFFSET(1) NUMBITS(1) [
            Block = 0,
            Table = 1
        ],

        VALID OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

// A level 3 page descriptor, as per ARMv8-A Architecture Reference Manual Figure D5-17.
register_bitfields! {u64,
    STAGE1_PAGE_DESCRIPTOR [
        /// Unprivileged execute-never.
        UXN      OFFSET(54) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Privileged execute-never.
        PXN      OFFSET(53) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Physical address of the next table descriptor (lvl2) or the page descriptor (lvl3).
        OUTPUT_ADDR_64KiB OFFSET(16) NUMBITS(32) [], // [47:16]

        /// Access flag.
        AF       OFFSET(10) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Shareability field.
        SH       OFFSET(8) NUMBITS(2) [
            OuterShareable = 0b10,
            InnerShareable = 0b11
        ],

        /// Access Permissions.
        AP       OFFSET(6) NUMBITS(2) [
            RW_EL1 = 0b00,
            RW_EL1_EL0 = 0b01,
            RO_EL1 = 0b10,
            RO_EL1_EL0 = 0b11
        ],

        /// Memory attributes index into the MAIR_EL1 register.
        AttrIndx OFFSET(2) NUMBITS(3) [],

        TYPE     OFFSET(1) NUMBITS(1) [
            Reserved_Invalid = 0,
            Page = 1
        ],

        VALID    OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

impl convert::From<AttributeFields> for tock_registers::fields::FieldValue<u64, STAGE1_PAGE_DESCRIPTOR::Register> {
    fn from(value: AttributeFields) -> Self {
        let mut desc = match value.memory_attributes {
            crate::memory::mmu::MemoryAttributes::CacheableDRAM => STAGE1_PAGE_DESCRIPTOR::SH::InnerShareable
                + STAGE1_PAGE_DESCRIPTOR::AttrIndx.val(crate::memory::mmu::arch_mmu::mair::NORMAL_WB_NT_RW),
            crate::memory::mmu::MemoryAttributes::Device => todo!(),
        };

        desc
    }
}

#[derive(Copy, Clone)]
struct PageDescriptor {
    value: usize,
}

impl PageDescriptor {
    const fn zero() -> Self {
        Self {value: 0}
    }
}

#[derive(Copy, Clone)]
struct TableDescriptor {
    value: usize,
}

impl TableDescriptor {
    const fn zero() -> Self {
        Self {value: 0}
    }
}

#[repr(C, align(65536))]
pub struct TranslationTable<const NUM_TABLES: usize> {
    lower_level3: [[PageDescriptor; 8192]; NUM_TABLES],
    lower_level2: [TableDescriptor; NUM_TABLES],
}

impl<const NUM_TABLES: usize> TranslationTable<NUM_TABLES> {
    pub const fn new() -> Self {
        Self {
            lower_level3: [[PageDescriptor::zero(); 8192]; NUM_TABLES],
            lower_level2: [TableDescriptor::zero(); NUM_TABLES],
        }
    }

    pub fn populate_tables(&mut self, descriptions: &[TranslationDescription]) {
        
    }

}
