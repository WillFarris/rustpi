.section .text._start

.globl _start
.type _start, function
_start:
    mrs x5, mpidr_el1
    and x5, x5, #0x3

    mov x4, #0x20000
    mul x3, x4, x5
    add x4, x3, x4
    
    msr sp_el1, x4
    sub x4, x4, #0x10000
    msr sp_el0, x4

    ldr x0, =0b00110000110100000000100000000000
    msr sctlr_el1, x0  

    //ldr x0, =HCR_VALUE
    //msr hcr_el2, x0

    //ldr x0, =SCR_VALUE
    //msr scr_el3, x0

    //ldr x0, =SPSR_EL3_TO_EL1h
    //msr spsr_el3, x0

    adr x0, el1_entry
    msr elr_el3, x0

    eret

el1_entry:
    //bl    irq_init_vectors
    //bl irq_enable
    //mov x0, #0x1
    //bl write_pmcr_el0

    cmp     x5, #0
    bne     slave_core_sleep

    adr     x0, bss_begin
    adr     x1, bss_end
    sub     x1, x1, x0
    bl      memzero
    bl      kernel_main
    b       hang

.balign 4
slave_core_sleep:
    wfe
	mov	    x2, 204
	movk    x2, 0x4000, lsl 16 //0x400000CC
	mrs     x0, mpidr_el1
	ubfiz   x0, x0, 4, 4
	ldr	    w1, [x0, x2]
	cbz     w1, slave_core_sleep
    str     w1, [x0, x2]
    
    dmb     sy // data memory buffer
    blr     x1 //branch and link to register
    dmb     sy
    b       slave_core_sleep
    ret

.globl core_execute
core_execute:
    dmb     sy
    ubfiz   x0, x0, 2, 8
    mov     x2, 140
    movk    x2, 0x4000, lsl 16
    str     w1, [x2, x0, lsl 2]
    sev
    dmb     sy
    ret

.globl memzero
memzero:
    str     xzr, [x0], #8
    subs    x1, x1, #8
    bgt     memzero
    ret