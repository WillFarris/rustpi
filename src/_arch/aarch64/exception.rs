use aarch64_cpu::registers::{ESR_EL1, SPSR_EL1};
use tock_registers::registers::InMemoryRegister;
use tock_registers::interfaces::{Readable, Writeable};

use crate::{info, warn};

#[repr(transparent)]
struct SpsrEL1(InMemoryRegister<u64, SPSR_EL1::Register>);
struct EsrEL1(InMemoryRegister<u64, ESR_EL1::Register>);

#[repr(C)]
struct ExceptionContext {
    regs: [u64; 30],
    lr: u64,
    elr_el1: u64,
    spsr_el1: SpsrEL1,
    esr_el1: EsrEL1,
}

#[no_mangle]
pub unsafe fn handle_irq() {
    let core = crate::utils::get_core();
    let core_irq_source = crate::bsp::QA7_REGS.get_incoming_irqs(core);

    if core_irq_source & 0b10 != 0 {
        let freq = aarch64_cpu::registers::CNTFRQ_EL0.get();
        aarch64_cpu::registers::CNTP_TVAL_EL0.set(freq / 10000);
        crate::scheduler::PTABLE.schedule();
    }
}

#[panic_handler]
pub unsafe fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    warn!(" ~ UwU we panic now ~\n{:?}", panic_info);
    loop {}
}