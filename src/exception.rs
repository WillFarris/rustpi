use core::arch::global_asm;
use aarch64_cpu::registers::VBAR_EL1;
use tock_registers::interfaces::Writeable;

use crate::println;

global_asm!(include_str!("exception.s"));

const EXCEPTION_ERROR_MESSAGES: [&'static str; 16] = [
    "SYNC_INVALID_EL1t",
    "IRQ_INVALID_EL1t",
    "FIQ_INVALID_EL1t",
    "ERROR_INVALID_EL1t",

    "SYNC_INVALID_EL1h",
    "IRQ_INVALID_EL1h",
    "FIQ_INVALID_EL1h",
    "ERROR_INVALID_EL1h",

    "SYNC_INVALID_EL0_64",
    "IRQ_INVALID_EL0_64",
    "FIQ_INVALID_EL0_64",
    "ERROR_INVALID_EL0_64",

    "SYNC_INVALID_EL0_32",
    "IRQ_INVALID_EL0_32",
    "FIQ_INVALID_EL0_32",
    "ERROR_INVALID_EL0_32",
];

#[no_mangle]
pub fn show_invalid_entry_message(exception_type: usize, esr_el1: usize, elr_el1: usize) {
    println!("invalid exception: {}, ESR_EL1: {}, ELR_EL1: {}", EXCEPTION_ERROR_MESSAGES[exception_type], esr_el1, elr_el1);
    loop {}
}

pub unsafe fn _init_vectors(_addr: u64) {
    VBAR_EL1.set(0);
}

#[no_mangle]
pub unsafe fn handle_irq() {

}