use core::arch::global_asm;
use aarch64_cpu::registers::*;

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

global_asm!(include_str!("start/start.s"));

extern "C" {
    fn irq_init_vectors();
    fn slave_core_sleep();
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

    //TODO: Zero BSS section
    
    crate::kernel_main()
}