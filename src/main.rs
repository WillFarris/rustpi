#![no_main]
#![no_std]
#![allow(unstable_features)]
#![feature(format_args_nl)]

mod bsp;
mod console;
mod exception;
mod memory;
mod print;
mod start;
mod synchronization;
mod tasks;
mod time;
mod utils;

use time::time_manager;
use utils::{get_core, get_el};
use tock_registers::interfaces::{Readable, Writeable};

#[no_mangle]
pub fn kernel_main() -> ! {

    crate::memory::mmu::map_translation_table();
    crate::memory::mmu::enable_mmu_and_caching();

    bsp::driver::init();

    println!();

    info!("Booting Raspberry Pi 3 in EL{}", get_el());
    info!("Timer resolution: {}ns", time_manager().resolution().as_nanos());

    
    let freq = aarch64_cpu::registers::CNTFRQ_EL0.get();
    let timer = freq / 10;
    aarch64_cpu::registers::CNTP_TVAL_EL0.set(timer);
    aarch64_cpu::registers::CNTP_CTL_EL0.write(aarch64_cpu::registers::CNTP_CTL_EL0::ENABLE::SET);

    bsp::raspberrypi::QA7_REGS.enable_core_timer_irqs();

    bsp::memory::virt_mem_layout().print_layout_info();

    exception::irq_enable();

    tasks::shell::shell();

    loop {}
}