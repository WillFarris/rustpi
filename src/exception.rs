use core::arch::global_asm;
use aarch64_cpu::registers::{CNTFRQ_EL0, CNTP_TVAL_EL0};
use tock_registers::interfaces::{Writeable, Readable};

use crate::{bsp::raspberrypi::QA7_REGS, info, print, println, utils::get_core};

global_asm!(include_str!("exception.s"));

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/exception.rs"]
mod arch_exception;

pub use arch_exception::*;

const EXCEPTION_ERROR_MESSAGES: [&str; 16] = [
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
pub fn show_invalid_entry_message(exception_type: usize, esr_el1: usize, elr_el1: usize, sp: usize) {
    println!("[core {}] invalid exception: {}, ESR_EL1: {:x}, ELR_EL1: {:x}\n\nRegister dump:", get_core(), EXCEPTION_ERROR_MESSAGES[exception_type], esr_el1, elr_el1);
    unsafe {
        let sp = *(sp as *const [u64; 32]);
        for i in 0..32 {
            print!("x{:<2}: 0x{:016X}  ", i, sp[i]);
            if (i+1) % 4 == 0{
                println!();
            }
        }
    }
    println!();
    loop {}
}

pub fn irq_enable() {
    unsafe {
        core::arch::asm!("msr daifclr, #2");
    }
}

pub fn irq_disable() {
    unsafe {
        core::arch::asm!("msr daifset, #2");
    }
}