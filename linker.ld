__rpi_phys_binary_load_addr = 0x80000;

SECTIONS
{
    /**************************************************
     *      Kernel Data                               *
     **************************************************/
     . = __rpi_phys_binary_load_addr;
    __text_start = .;
    .text : {
        KEEP(*(.text._start))
        *(.text*)
    }
	.rodata : {
        *(.rodata*)
    }
    __text_end = .;


    . = ALIGN(65536);
    __mapped_dram_start = .;
	.data : {
        *(.data*)
    }
	bss_begin = .;
	.bss : {
		 *(.bss*)
	}
	bss_end = .;
    . = ALIGN(65536);
    .locks : {
        LOCKS_START = .;
        *(.locks*)
        LOCKS_END = .;
    }

    /**************************************************
     *      Kernel Stack                              *
     **************************************************/
    .stack_core0 (NOLOAD) : {
        . = ALIGN(65536);
        __stack_start_core0__ = .;
        . = . + (2 * 65536);		/* 2 * 64KB kernel stack */
        __stack_end_core0__ = .;    
    }
	.stack_core1 (NOLOAD) : {
        . = ALIGN(65536);
        __stack_start_core1__ = .;
        . = . + (2 * 65536);		/* 2 * 64KB kernel stack */
        __stack_end_core1__ = .;    
    }
	.stack_core2 (NOLOAD) : {
        . = ALIGN(65536);
        __stack_start_core2__ = .;
        . = . + (2 * 65536);		/* 2 * 64KB kernel stack */
        __stack_end_core2__ = .;    
    }
	.stack_core3 (NOLOAD) : {
        . = ALIGN(65536);
        __stack_start_core3__ = .;
        . = . + (2 * 65536);		/* 2 * 64KB kernel stack */
        __stack_end_core3__ = .;    
    }

    .heap (NOLOAD) : {
        heap_start = .;
        . = . + 65536 * 1024;
        heap_end = .;
    }
    __mapped_dram_end = .;
    . = ALIGN(65536);

    __kernel_img_end = .;

    /DISCARD/ : {
        
    }
}