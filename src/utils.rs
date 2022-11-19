pub fn spin_for_cycles(n: usize) {
    for _ in 0..n {
        aarch64_cpu::asm::nop();
    }
}

extern "C" {
    fn get_core_asm() -> u8;
    fn get_el_asm() -> u8;
    pub fn u64_lock_acquire_asm(lock_addr: *const u64);
    pub fn u64_lock_release_asm(lock_addr: *const u64);
}

pub fn get_core() -> u8 {
    let mut core: usize = 0;
    unsafe {
        core::arch::asm!("mrs {}, mpidr_el1", out(reg) core)
    }
    (core & 0b11) as u8
}

pub fn get_el() -> u8 {
    let mut el: usize = 0;
    unsafe {
        core::arch::asm!("mrs {}, CurrentEL", out(reg) el)
    }
    ((el >> 2) & 0b11) as u8
}

pub fn _sys_timer_sleep_ms(ms: u64) {
    let start_time = _sys_timer_get_ticks();
    while _sys_timer_get_ticks() < start_time + (ms * 1000) {}
}

pub fn _sys_timer_get_ticks() -> u64 {
    let timer_address: usize = 0x3F003004;// LS bits of timer

    #[allow(unused_assignments)]
    let mut lo = 0;
    #[allow(unused_assignments)]
    let mut hi = 0;
    unsafe {
        core::arch::asm!("
            ldp {wlo}, {whi}, [{addr}]
        ", addr = in(reg) timer_address, wlo = out(reg) lo, whi = out(reg) hi);
    }
    lo | (hi << 32)
}