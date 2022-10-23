use core::arch::global_asm;

global_asm!(include_str!("start.s"));

#[no_mangle]
pub unsafe fn _rust_entry() -> ! {
    crate::kernel_main()
}