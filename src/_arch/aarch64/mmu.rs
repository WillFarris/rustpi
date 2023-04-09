use aarch64_cpu::registers::{TCR_EL1, MAIR_EL1, TTBR0_EL1, SCTLR_EL1};
use tock_registers::interfaces::{Writeable, ReadWriteable};
use crate::bsp::memory::virt_mem_layout;

use super::translation_table::TranslationTable;

const NUM_TABLES: usize = 3;


pub mod mair {
    pub const DEVICE: u64 = 0;
    pub const NORMAL_WB_NT_RW: u64 = 1;
    pub const NORMAL_NC: u64 = 4;
}


/*impl<const NUM_TABLES: usize> TranslationTable<NUM_TABLES> {
    const fn new() -> Self {
        Self {
            lower_level3: [[PageDescriptor::zero(); 8192]; NUM_TABLES],
            lower_level2: [TableDescriptor::zero(); NUM_TABLES],
        }
    }

    pub fn identity_map(&mut self) {
        /*let lock_start_addr = unsafe { &LOCKS_START as *const u8 as usize };
        let lock_end_addr = unsafe { &LOCKS_END as *const u8 as usize };*/

        for i in 0..NUM_TABLES {
            let lvl2_addr = ((&self.lower_level3[i] as *const PageDescriptor) as usize) >> 16;
            self.lower_level2[i].value = 
                (lvl2_addr << 16) | 
                (0b11 << 0);

            for j in 0..8192 {
                let virt_address = (i << 29) + (j << 16);
                let mut mair_attr = 4;

                /*if virt_address >= PBASE_START && virt_address <= PBASE_END {
                    mair_attr = 0;
                } else if virt_address >= lock_start_addr && virt_address <= lock_end_addr {
                    /* page w/ a lock, mark as non-cacheable */
                    //mair_attr = 1;
                    mair_attr = 4;
                }*/

                self.lower_level3[i][j].value = 
                    virt_address      | // Virtual address
                    (0b1       << 10) | // Accessed
                    (0b10      <<  8) | // Inner-sharable
                    (0b0       <<  7) | // Read-Write
                    (0b0       <<  6) | // Kernel only
                    (mair_attr <<  2) | // MAIR attribute index
                    (0b11      <<  0);  // valid page
                
                //crate::println!("Last entry in L3 table for L2 table {} at index {:#0x}: {:#0x}", i, j, self.lower_level3[i][j]);
            }
            //crate::println!("Address of translation_table: {:#0x}", unsafe {&TRANSLATION_TABLE as *const PageTable as usize});
        }
    }
}*/

#[no_mangle]
static mut TRANSLATION_TABLE: TranslationTable<3> = TranslationTable::new();

pub fn populate_tables() {

    let layout = virt_mem_layout();

    for region in layout {
        let name = region.name;
        let region_start = (region.physical_start)();
        assert_eq!(region_start & !0xFFFF, region_start);
        let region_end = (region.physical_end)();
        let last_page = region_end & !0xFFFF;

        // TODO: Support page sizes other than 64KiB
        let num_pages = 1 + (last_page - region_start) / 0x10000;

        /*unsafe {
            TRANSLATION_TABLE.lower_level2[0] = TableDescriptor {value:(&TRANSLATION_TABLE.lower_level3[0] as *const PageDescriptor) as usize};
        }*/

        crate::println!("mapping {}: {:#x} - {:#x} (last page {:#x})", name, region_start, region_end, last_page);
        for i in 0..num_pages {
            let page_address = region_start + i * 0x10000;
            crate::println!("Mapping {:#x}", page_address);

            let level2_index = (page_address >> 30) & 0x3FF;
            let level3_index = (page_address >> 16) & 0x3FFF;

            //let attributes: tock_registers::fields::FieldValue<u64, STAGE1_PAGE_DESCRIPTOR::Register> = region.attributes.into();
        }

    }
}

pub fn enable_mmu_and_caching() {

    // Set MAIR_EL1
    MAIR_EL1.write(
        MAIR_EL1::Attr0_Device::nonGathering_nonReordering_noEarlyWriteAck +
        MAIR_EL1::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc +
        MAIR_EL1::Attr1_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc +
        MAIR_EL1::Attr4_Normal_Inner::NonCacheable +
        MAIR_EL1::Attr4_Normal_Outer::NonCacheable
    );

    let t0sz = 32;

    // Set TCR_EL1
    TCR_EL1.write(TCR_EL1::TBI0::Used +
        TCR_EL1::IPS::Bits_40 + 
        TCR_EL1::TG0::KiB_64 +
        TCR_EL1::SH0::Outer +
        TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable +
        TCR_EL1::IRGN0::WriteBack_ReadAlloc_NoWriteAlloc_Cacheable +
        TCR_EL1::EPD0::EnableTTBR0Walks +
        TCR_EL1::A1::TTBR0 +
        TCR_EL1::EPD1::DisableTTBR1Walks +
        TCR_EL1::T0SZ.val(t0sz)
    );

    // Set TTBR1_EL1 to point to translation_table.higher_level2
    /*TTBR1_EL1.set(
        unsafe {
            &TRANSLATION_TABLE.higher_level2 as *const [usize; 8192] as u64
        }
    );*/

    // Set TTBR0_EL1 to point to translation_table.lower_level2;
    /*TTBR0_EL1.set_baddr(
        unsafe {
            &TRANSLATION_TABLE.lower_level2 as *const [TableDescriptor; NUM_TABLES] as u64
        }
    );*/

    aarch64_cpu::asm::barrier::isb(aarch64_cpu::asm::barrier::SY);

    // Set d-cache, i-cache, mmu enable bits of SCTLR_EL1
    SCTLR_EL1.modify(
        SCTLR_EL1::C::Cacheable +
        SCTLR_EL1::I::Cacheable +
        SCTLR_EL1::M::Enable
    );
    
    aarch64_cpu::asm::barrier::isb(aarch64_cpu::asm::barrier::SY);

}

