use core::arch::global_asm;
use cortex_a::registers::*;

#[no_mangle]
static SCTLR_INIT_VAL: u64 = SCTLR_EL1::EE::LittleEndian.value | SCTLR_EL1::NAA::Disable.value | SCTLR_EL1::I::NonCacheable.value | SCTLR_EL1::C::NonCacheable.value | SCTLR_EL1::M::Disable.value;
#[no_mangle]
static HCR_INIT_VAL: u64 = HCR_EL2::RW::EL1IsAarch64.value;
#[no_mangle]
static SCR_INIT_VAL: u64 = SCR_EL3::RW::NextELIsAarch64.value | SCR_EL3::NS::NonSecure.value;
#[no_mangle]
static SPSR_EL3_INIT_VAL: u64 = SPSR_EL3::M::EL1h.value;

global_asm!(include_str!("start.s"));

#[no_mangle]
pub unsafe extern "C" fn _rust_entry() -> ! {
    crate::kernel_main()
}