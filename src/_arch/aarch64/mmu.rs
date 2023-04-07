use aarch64_cpu::registers::{TCR_EL1, MAIR_EL1, TTBR0_EL1, TTBR1_EL1, SCTLR_EL1};
use tock_registers::interfaces::Writeable;
use crate::bsp::memory::virt_mem_layout;

const NUM_TABLES: usize = 3;

/*extern "C" {
    static LOCKS_START: u8;
    static LOCKS_END: u8;
}*/

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
struct TranslationTable {
    lower_level3: [[PageDescriptor; 8192]; NUM_TABLES],
    lower_level2: [TableDescriptor; NUM_TABLES],

    /*higher_level3: [[usize; 8192]; NUM_TABLES],
    higher_level2: [usize; 8192],*/
}

impl TranslationTable {
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
}

#[no_mangle]
static mut TRANSLATION_TABLE: TranslationTable = TranslationTable::new();

pub fn init_translation_tables() {

    let layout = virt_mem_layout();

    for region in layout {
        let name = region.name;
        let from = region.physical_start;
        let from = from();
        let to = region.physical_end;
        let to = to();

        crate::println!("{}: {:#x} - {:#x} ", name, from, to);
    }

    todo!("Translation Table apping")
    
    
    /*unsafe {
        TRANSLATION_TABLE.identity_map();
    }*/
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

    // Set TCR_EL1
    let tcr_el1 = (
        TCR_EL1::TBI0::Used +
        TCR_EL1::IPS::Bits_40 + 
        TCR_EL1::TG0::KiB_64 +
        TCR_EL1::SH0::Outer +
        TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable +
        TCR_EL1::IRGN0::WriteBack_ReadAlloc_NoWriteAlloc_Cacheable +
        TCR_EL1::EPD0::EnableTTBR0Walks +
        TCR_EL1::A1::TTBR0 +
        TCR_EL1::EPD1::DisableTTBR1Walks
    ).value | 32;
    TCR_EL1.set(tcr_el1);

    // Set TTBR1_EL1 to point to translation_table.higher_level2
    /*TTBR1_EL1.set(
        unsafe {
            &TRANSLATION_TABLE.higher_level2 as *const [usize; 8192] as u64
        }
    );*/

    // Set TTBR0_EL1 to point to translation_table.lower_level2;
    TTBR0_EL1.set(
        unsafe {
            &TRANSLATION_TABLE.lower_level2 as *const [TableDescriptor; NUM_TABLES] as u64
        }
    );

    // Set d-cache, i-cache, mmu enable bits of SCTLR_EL1
    SCTLR_EL1.write(
        SCTLR_EL1::C::Cacheable +
        SCTLR_EL1::I::Cacheable +
        SCTLR_EL1::M::Enable
    );
    

    // Invalidate TLB, vmalle1
    unsafe {
        core::arch::asm!("
            isb
            tlbi vmalle1
            dsb sy
            isb
        ");
    }

}

