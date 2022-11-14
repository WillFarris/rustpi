pub fn spin_for_cycles(n: usize) {
    for _ in 0..n {
        aarch64_cpu::asm::nop();
    }
}

extern "C" {
    fn get_core_asm() -> u8;
    fn get_el_asm() -> u8;
}

pub fn get_core() -> u8 {
    unsafe {get_core_asm()}
}

pub fn get_el() -> u8 {
    unsafe {get_el_asm()}
}