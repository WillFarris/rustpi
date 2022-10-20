.section .text._start

.globl _start
.type _start, function
_start:
    mrs     x0, s3_1_c15_c2_1
    orr     x0, x0, #0x40
    msr     s3_1_c15_c2_1, x0

    mrs     x5, mpidr_el1
    and     x5, x5, #0x3

    cbnz    x5, hang

    cmp     x5, #0
    beq     core0_stack
    cmp     x5, #1
    beq     core1_stack
    cmp     x5, #2
    beq     core2_stack
    cmp     x5, #3
    beq     core3_stack

hang:
    wfe
    b       hang

core0_stack:
    adr     x2, __EL0_stack_core0
    adr     x3, __EL1_stack_core0
    adr     x4, __EL2_stack_core0
    b       set_stack
core1_stack:
    adr     x2, __EL0_stack_core1
    adr     x3, __EL1_stack_core1
    adr     x4, __EL2_stack_core1
    b       set_stack
core2_stack:
    adr     x2, __EL0_stack_core2
    adr     x3, __EL1_stack_core2
    adr     x4, __EL2_stack_core2
    b       set_stack
core3_stack:
    adr     x2, __EL0_stack_core3
    adr     x3, __EL1_stack_core3
    adr     x4, __EL2_stack_core3
    b       set_stack

set_stack:
    mrs     x0, CurrentEL
    msr     sp_el0, x2
    msr     sp_el1, x3
    msr     sp_el2, x4

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