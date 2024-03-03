use core::arch::global_asm;
use aarch64_cpu::registers::{SCTLR_EL1, HCR_EL2, SCR_EL3, CPACR_EL1, SPSR_EL3};
use core::cell::UnsafeCell;

use crate::get_core;

#[no_mangle]
static SCTLR_INIT_VAL: u64 = SCTLR_EL1::EE::LittleEndian.value | SCTLR_EL1::NAA::Disable.value | SCTLR_EL1::I::NonCacheable.value | SCTLR_EL1::C::NonCacheable.value | SCTLR_EL1::M::Disable.value;
#[no_mangle]
static HCR_INIT_VAL: u64 = HCR_EL2::RW::EL1IsAarch64.value;
#[no_mangle]
static SCR_INIT_VAL: u64 = SCR_EL3::RW::NextELIsAarch64.value | SCR_EL3::NS::NonSecure.value;
#[no_mangle]
static SPSR_EL3_INIT_VAL: u64 = SPSR_EL3::D::Masked.value | SPSR_EL3::A::Masked.value | SPSR_EL3::I::Masked.value | SPSR_EL3::F::Masked.value | SPSR_EL3::M::EL1h.value;
#[no_mangle]
static CPACR_EL1_INIT_VAL: u64 = CPACR_EL1::FPEN::TrapNothing.value;

global_asm!(include_str!("cpu/start.s"));

extern "C" {
    fn irq_init_vectors();
    fn slave_core_sleep();
    fn memzero(addr: usize, length: usize);
}

extern "Rust" {
    static bss_begin: UnsafeCell<()>;
    static bss_end: UnsafeCell<()>;
}

pub fn _hang() -> ! {
    loop {
        aarch64_cpu::asm::wfe();
    }
}

#[no_mangle]
pub unsafe extern "C" fn _el1_rust_entry() -> ! {
    irq_init_vectors();

    if get_core() != 0 {
        slave_core_sleep()
    }

    let bss_start = bss_begin.get() as usize;
    let bss_length = bss_end.get() as usize - bss_start;
    memzero(bss_start, bss_length);
    
    crate::kernel_main()
}