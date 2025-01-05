#![no_main]
#![no_std]
#![allow(unstable_features)]
#![feature(format_args_nl)]

mod bsp;
mod console;
mod exception;
mod memory;
mod print;
mod scheduler;
mod start;
mod synchronization;
mod tasks;
mod time;
mod utils;

extern crate alloc;

use time::time_manager;
use utils::{get_core, get_el};
use tock_registers::interfaces::{Readable, Writeable};

extern "C" {
    fn core_execute(core: u8, f: extern "C" fn());
}

extern "C" fn init_core() {
    memory::mmu::enable_mmu_and_caching();
    let freq = aarch64_cpu::registers::CNTFRQ_EL0.get();
    aarch64_cpu::registers::CNTP_TVAL_EL0.set(freq / 100);
    aarch64_cpu::registers::CNTP_CTL_EL0.write(aarch64_cpu::registers::CNTP_CTL_EL0::ENABLE::SET);
    scheduler::PTABLE.init_core();
    bsp::raspberrypi::QA7_REGS.enable_core_timer_irqs();
    exception::irq_enable();
}

#[no_mangle]
pub fn kernel_main() -> ! {

    crate::memory::mmu::map_translation_table();
    crate::memory::mmu::enable_mmu_and_caching();
    crate::memory::init_heap();

    bsp::driver::init();

    println!();

    info!("Booting Raspberry Pi 3 in EL{}", get_el());
    info!("Timer resolution: {}ns", time_manager().resolution().as_nanos());

    let mailbox_regs = bsp::device_driver::MailboxInterface::new(0x3F00B880);

    unsafe {
        //let mbox = &mut MAILBOX.0;
        let mut mbox = alloc::boxed::Box::new([0u32; 7]);

        mbox[0] = 8*4;
        mbox[1] = 0;

        mbox[2] = 0x00010005;
        mbox[3] = 8;
        mbox[4] = 8;
        mbox[5] = 0;
        mbox[6] = 0;

        let ptr = mbox.as_ptr();

        core::arch::asm!(
            "
            dsb sy
            dmb sy
            dc ivac, {addr}
            ",
            addr = in(reg) ptr,
        );

        mailbox_regs.call(8, &*mbox);

        core::arch::asm!(
            "
            dsb sy
            dmb sy
            dc ivac, {addr}
            ",
            addr = in(reg) ptr,
        );

        info!("DRAM size: {} MiB", mbox[6] / (1024 * 1024));
    }
    
    let freq = aarch64_cpu::registers::CNTFRQ_EL0.get();
    aarch64_cpu::registers::CNTP_TVAL_EL0.set(freq / 100);
    aarch64_cpu::registers::CNTP_CTL_EL0.write(aarch64_cpu::registers::CNTP_CTL_EL0::ENABLE::SET);

    bsp::memory::virt_mem_layout().print_layout_info();

    bsp::raspberrypi::QA7_REGS.enable_core_timer_irqs();

    unsafe {
        core_execute(1, init_core);
    }

    tasks::register_cmd("ptable", || {
        scheduler::PTABLE.print();
    });

    tasks::register_cmd("test_loop", || {
        let max = 10;
        for i in 0..max {
            bsp::system_timer().wait_for_ms(1000);
            println!("loop {}/{}", i + 1, max);
        }
    });

    tasks::register_cmd("loop_forever", || {
        let mut c = 0;
        loop {
            bsp::system_timer().wait_for_ms(5000);
            c += 1;
            info!("loop {}", c);
        }
    });

    tasks::register_cmd("uptime", || {
        let ticks = bsp::system_timer().get_ticks();
        let ms = ticks / 1000;
        let s = ms / 1000;
        let m = s / 60;
        let h = m / 60;
        let d = h / 24;
        crate::println!("uptime: {}d {}h {}m {}s", d, h % 24, m % 60, s % 60);
    });

    
    scheduler::PTABLE.init_core();
    scheduler::PTABLE.new_process("shell", tasks::shell::shell);

    exception::irq_enable();

    loop {}
}