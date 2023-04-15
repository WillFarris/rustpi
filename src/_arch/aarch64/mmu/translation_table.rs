use core::convert;
use tock_registers::{register_bitfields, registers::InMemoryRegister};
use tock_registers::interfaces::{Readable, Writeable};

use crate::bsp;
use crate::memory::mmu::{AttributeFields, TranslationGranule, TranslationDescription};

pub type Granule512MiB = TranslationGranule<{ 512 * 1024 * 1024 }>;
pub type Granule64KiB = TranslationGranule<{ 64 * 1024 }>;

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
    value: u64,
}

impl PageDescriptor {
    const fn zero() -> Self {
        Self {value: 0}
    }
}

#[derive(Copy, Clone)]
struct TableDescriptor {
    value: u64,
}

impl TableDescriptor {
    const fn zero() -> Self {
        Self {value: 0}
    }

    fn from_next_level_table_addr(phys_next_lvl_table_addr: usize) -> Self {
        let mut val = InMemoryRegister::<u64, STAGE1_TABLE_DESCRIPTOR::Register>::new(0);

        let shifted = phys_next_lvl_table_addr >> Granule64KiB::SHIFT;
        val.write(
            STAGE1_TABLE_DESCRIPTOR::NEXT_LEVEL_TABLE_ADDR_64KiB.val(shifted as u64)
                + STAGE1_TABLE_DESCRIPTOR::TYPE::Table
                + STAGE1_TABLE_DESCRIPTOR::VALID::True,
        );

        Self { value: val.get() }
    }
}

trait StartAddr {
    fn phys_start_addr_usize(&self) -> usize;
}

impl<T, const N: usize> StartAddr for [T; N] {
    fn phys_start_addr_usize(&self) -> usize {
        self as *const _ as usize
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

    pub fn populate_tables(&mut self) {

        for (level2_num, level2_entry) in self.lower_level2.iter_mut().enumerate() {
            *level2_entry = TableDescriptor::from_next_level_table_addr(self.lower_level3[level2_num].phys_start_addr_usize());

            for (level3_num, level3_entry) in self.lower_level3[level2_num].iter_mut().enumerate() {
                let virt_addr = (level2_num << Granule512MiB::SHIFT) + (level3_num << Granule64KiB::SHIFT);

                //let layout = bsp::memory::mmu::virt_mem_layout();

                //let (phys_output_addr, attribute_fields) = bsp::memory::mmu::virt_mem_layout().virt_addr_properties(virt_addr)?;
                //*level3_entry = PageDescriptor::from_output_addr(phys_output_addr, &attribute_fields);


            }
        }
        
    }

}
