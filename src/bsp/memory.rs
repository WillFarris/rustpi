
pub mod mmu {

    use aarch64_cpu::registers::{TCR_EL1, MAIR_EL1, TTBR0_EL1, TTBR1_EL1, SCTLR_EL1};
    use tock_registers::interfaces::Writeable;

    use crate::bsp::raspberrypi::{PBASE_START, PBASE_END};

    const NUM_TABLES: usize = 3;

    #[repr(C)]
    struct PageTable {
        lower_level3: [[usize; 8192]; NUM_TABLES],
        higher_level3: [[usize; 8192]; NUM_TABLES],
        lower_level2: [usize; 8192],
        higher_level2: [usize; 8192],
    }
    
    impl PageTable {
        const fn new() -> Self {
            Self {
                lower_level3: [[0usize; 8192]; NUM_TABLES],
                higher_level3: [[0usize; 8192]; NUM_TABLES],
                lower_level2: [0usize; 8192],
                higher_level2: [0usize; 8192],
            }
        }

        pub fn identity_map(&mut self) {
            for i in 0..NUM_TABLES {
                let lvl2_address = (&self.lower_level3[i] as *const usize) as usize;
                let lvl2_address_offset = lvl2_address >> 16;
                self.higher_level2[i] = (lvl2_address_offset << 16) | (0b11 << 0);

                for j in 0..8192 {
                    let virt_address = (i << 29) + (j << 16);
                    let mut mair_attr = 4;

                    if virt_address > PBASE_START && virt_address < PBASE_END {
                        mair_attr = 0;
                    } /* TODO: else if virt_address == a page w/ a lock, mark as non-cacheable */

                    self.lower_level3[i][j] = 
                        virt_address      | // Virtual address
                        (0b1       << 10) | // Accessed
                        (0b11      <<  8) | // Inner-sharable
                        (0b0       <<  7) | // Read-Write
                        (0b0       <<  6) | // Kernel only
                        (mair_attr <<  2) | // MAIR attribute index
                        (0b11      <<  0)   // valid page
                    ;
                }
            }
        }
    }

    static mut TRANSLATION_TABLE: PageTable = PageTable::new();

    pub fn init() {
        //TODO: Wrap TRANSLATION_TABLE with lock? But we can't use atomic locks yet...
        unsafe {
            TRANSLATION_TABLE.identity_map();
        }

        // Set TCR_EL1
        TCR_EL1.write(
            TCR_EL1::TBI0::Used +
            TCR_EL1::IPS::Bits_40 + 
            TCR_EL1::TG0::KiB_64 +
            TCR_EL1::SH0::Outer +
            TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable +
            TCR_EL1::IRGN0::WriteBack_ReadAlloc_NoWriteAlloc_Cacheable +
            TCR_EL1::EPD0::EnableTTBR0Walks +
            TCR_EL1::A1::TTBR0 +
            TCR_EL1::EPD1::DisableTTBR1Walks
        );
        

        // Set MAIR_EL1
        MAIR_EL1.write(
            MAIR_EL1::Attr0_Device::nonGathering_nonReordering_noEarlyWriteAck +
            MAIR_EL1::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc +
            MAIR_EL1::Attr1_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc +
            MAIR_EL1::Attr4_Normal_Inner::NonCacheable +
            MAIR_EL1::Attr4_Normal_Outer::NonCacheable
        );

        // Set TTBR1_EL1 to point to translation_table.higher_level2
        TTBR1_EL1.set(
            unsafe {
                &TRANSLATION_TABLE.higher_level2 as *const [usize; 8192] as u64
            }
        );

        // Set TTBR0_EL1 to point to translation_table.lower_level2;
        TTBR0_EL1.set(
            unsafe {
                &TRANSLATION_TABLE.lower_level2 as *const [usize; 8192] as u64
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
            core::arch::asm!("tlbi vmalle1");
        }

    }

    
}