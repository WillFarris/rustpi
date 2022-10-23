use core::arch::{global_asm, asm};


global_asm!(include_str!("start.s"));

use cortex_a::registers::*;

#[no_mangle]
pub unsafe extern "C" fn _rust_entry() -> ! {

    let mut sctlr_val =
        SCTLR_EL1::EE::LittleEndian +
        SCTLR_EL1::NAA::Disable +
        SCTLR_EL1::I::NonCacheable +
        SCTLR_EL1::C::NonCacheable +
        SCTLR_EL1::M::Disable;
    asm!("msr sctlr_el1, {}", out(reg) sctlr_val.value);
    

    crate::kernel_main()
    // Return from exception to EL1
    //cortex_a::asm::eret()
}