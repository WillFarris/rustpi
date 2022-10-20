#![no_main]
#![no_std]

mod start;
mod bcm2xxx_pl011_uart;
mod bcm2xxx_gpio;


const PBASE: u64 = 0x3F000000;
const AUX_REGS_ADDR: u64 = (PBASE + 0x00215000);

#[no_mangle]
pub unsafe fn kernel_main() {
    let auxregs = AUX_REGS_ADDR as *mut AuxRegs;
    
    let t = (*auxregs).mu_io;
    
    while ((*auxregs).mu_lsr & 0x20) == 0 {
        
    }
    (*auxregs).mu_io = 0x64;
    
    loop {

    }
}

#[panic_handler]
pub unsafe fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {

    }
}