SECTIONS
{
    . = 0x80000;

    __stack_high = .;
	
    .text : { *(.text._start) *(.text) }
	.rodata : { *(.rodata) }
	.data : { 
        *(.data)
    }
	. = ALIGN(16);
	bss_begin = .;
	.bss : {
		 *(.bss*)
	} 
	bss_end = .;
    __kernel_img_end = .;

}