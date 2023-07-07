use aarch64_cpu::registers::{TCR_EL1, MAIR_EL1, TTBR0_EL1, SCTLR_EL1};
use tock_registers::interfaces::{Writeable, ReadWriteable};
use crate::bsp::memory::virt_mem_layout;

use super::translation_table::TranslationTable;

const NUM_TABLES: usize = 3;


pub mod mair {
    pub const DEVICE: u64 = 0;
    pub const NORMAL_WB_NT_RW: u64 = 1;
    pub const _NORMAL_NC: u64 = 4;
}

#[no_mangle]
static mut TRANSLATION_TABLE: TranslationTable<3> = TranslationTable::new();


pub fn enable_mmu_and_caching() {

    unsafe { TRANSLATION_TABLE.populate_tables() };

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

    // Set TTBR1_EL1 to point to translation_table.higher_level2 (not in use atm)
    /*TTBR1_EL1.set(
        unsafe {
            &TRANSLATION_TABLE.higher_level2 as *const [usize; 8192] as u64
        }
    );*/

    // Set TTBR0_EL1 to point to translation_table.lower_level2;
    TTBR0_EL1.set_baddr(
        unsafe {
            TRANSLATION_TABLE.phys_base_address()
        }
    );

    aarch64_cpu::asm::barrier::isb(aarch64_cpu::asm::barrier::SY);

    // Set d-cache, i-cache, mmu enable bits of SCTLR_EL1
    SCTLR_EL1.modify(
        SCTLR_EL1::C::Cacheable +
        SCTLR_EL1::I::Cacheable +
        SCTLR_EL1::M::Enable
    );
    
    aarch64_cpu::asm::barrier::isb(aarch64_cpu::asm::barrier::SY);

}

