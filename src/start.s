.section .text._start

.globl _start
.type _start, function
_start:
    mrs     x0, s3_1_c15_c2_1
    orr     x0, x0, #0x40
    msr     s3_1_c15_c2_1, x0

    mrs     x5, mpidr_el1
    and     x5, x5, #0x3

    cmp     x5, #0
    beq     core0_stack
    cmp     x5, #1
    beq     core1_stack
    cmp     x5, #2
    beq     core2_stack
    cmp     x5, #3
    beq     core3_stack

core0_stack:
    adr     x1, __EL0_stack_core0
    adr     x2, __EL1_stack_core0
    adr     x3, __EL2_stack_core0
    adr     x4, __EL3_stack_core0
    b       set_stack
core1_stack:
    adr     x1, __EL0_stack_core1
    adr     x2, __EL1_stack_core1
    adr     x3, __EL2_stack_core1
    adr     x4, __EL3_stack_core1
    b       set_stack
core2_stack:
    adr     x1, __EL0_stack_core2
    adr     x2, __EL1_stack_core2
    adr     x3, __EL2_stack_core2
    adr     x4, __EL3_stack_core2
    b       set_stack
core3_stack:
    adr     x1, __EL0_stack_core3
    adr     x2, __EL1_stack_core3
    adr     x3, __EL2_stack_core3
    adr     x4, __EL3_stack_core3
    b       set_stack

set_stack:
    msr     sp_el0, x1
    msr     sp_el1, x2
    msr     sp_el2, x3
    mov     sp, x4

    adr     x0, SCTLR_INIT_VAL
    ldr     x0, [x0]
    msr     sctlr_el1, x0
    
    adr     x0, HCR_INIT_VAL
    ldr     x0, [x0]
    msr     hcr_el2, x0
    
    adr     x0, SCR_INIT_VAL
    ldr     x0, [x0]
    msr     scr_el3, x0

    adr     x0, CPACR_EL1_INIT_VAL
    ldr     x0, [x0]
    msr     cpacr_el1, x0

    adr     x0, SPSR_EL3_INIT_VAL
    ldr     x0, [x0]
    msr     spsr_el3, x0

    adr     x0, _el1_rust_entry
    msr     elr_el3, x0

    eret

.balign 4
.globl slave_core_sleep
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

.globl get_el_asm
get_el_asm:
    mrs     x0, CurrentEL
    lsr     x0, x0, #2
    and     x0, x0, 0b11
    ret

.globl get_core_asm
get_core_asm:
    mrs     x0, mpidr_el1
    and     x0, x0, 0b11
    ret

.globl memzero
memzero:
    str     xzr, [x0], #8
    subs    x1, x1, #8
    bgt     memzero
    ret

.globl irq_init_vectors
irq_init_vectors:
    adr     x0, vectors
    msr     vbar_el1, x0
    ret

.globl irq_enable
irq_enable:
    msr     daifclr, #2
    ret

.globl irq_disable
irq_disable:
    msr     daifset, #2
    ret

.globl u64_lock_acquire_asm
u64_lock_acquire_asm:
.p2align 8
    msr     daifset, 0b0010
    mov     x2, #1
    mrs     x2, mpidr_el1
    and     x2, x2, 0b11
    add     x2, x2, #1
.p2align 2
.L4:
    ldaxr   x1, [x0]
    stlxr   w3, x2, [x0]
    cbnz    w3, .L4
    uxtb    w1, w1
    cbnz    w1, .L4
    ret

.globl u64_lock_release_asm
u64_lock_release_asm:
    stlr    xzr, [x0]
    msr     daifclr, 0b0010
    ret