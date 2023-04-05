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
    __data_start = .;
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
    __data_end = .;

    /**************************************************
     *      Kernel Stack                              *
     **************************************************/
    . = ALIGN(65536);
    __stack_start = .;
    .stack_core0 (NOLOAD) : {
        . = ALIGN(16);
        __stack_start_core0__ = .;
        . = . + 1024;				/* 1KB EL0 stack */
        __EL0_stack_core0 = .;
        . = . + 65536;				/* 64KB EL1 stack */
        __EL1_stack_core0 = .;
        . = . + 512;				/* 512B EL2 stack */
        __EL2_stack_core0 = .;
        . = . + 512;				/* 512B EL3 stack */
        __EL3_stack_core0 = .;
        __stack_end_core0__ = .;    
    }
	.stack_core1 (NOLOAD) : {
        . = ALIGN(16);
        __stack_start_core1__ = .;
        . = . + 1024;				/* 1KB EL0 stack */
        __EL0_stack_core1 = .;
        . = . + 65536;				/* 64KB EL1 stack */
        __EL1_stack_core1 = .;
        . = . + 512;				/* 512B EL2 stack */
        __EL2_stack_core1 = .;
        . = . + 512;				/* 512B EL3 stack */
        __EL3_stack_core1 = .;
        __stack_end_core1__ = .;    
    }
	.stack_core2 (NOLOAD) : {
        . = ALIGN(16);
        __stack_start_core2__ = .;
        . = . + 1024;				/* 1KB EL0 stack */
        __EL0_stack_core2 = .;
        . = . + 65536;				/* 64KB EL1 stack */
        __EL1_stack_core2 = .;
        . = . + 512;				/* 512B EL2 stack */
        __EL2_stack_core2 = .;
        . = . + 512;				/* 512B EL3 stack */
        __EL3_stack_core2 = .;
        __stack_end_core2__ = .;    
    }
	.stack_core3 (NOLOAD) : {
        . = ALIGN(16);
        __stack_start_core3__ = .;
        . = . + 1024;				/* 1KB EL0 stack */
        __EL0_stack_core3 = .;
        . = . + 65536;				/* 64KB EL1 stack */
        __EL1_stack_core3 = .;
        . = . + 512;				/* 512B EL2 stack */
        __EL2_stack_core3 = .;
        . = . + 512;				/* 512B EL3 stack */
        __EL3_stack_core3 = .;
        __stack_end_core3__ = .;    
    }
    __stack_end = .;

    . = ALIGN(65536);
    __heap_start = .;
    .heap (NOLOAD) : {
        heap_start = .;
        . = . + 0x1000 * 1024;
        heap_end = .;
    }
    __heap_end = .;

    __kernel_img_end = .;

    /DISCARD/ : {
        
    }
}