#include "entry.h"

.macro	kernel_entry
   sub     sp, sp, #16 * 17
   stp     x0, x1, [sp, #16 * 0]
   stp     x2, x3, [sp, #16 * 1]
   stp     x4, x5, [sp, #16 * 2]
   stp     x6, x7, [sp, #16 * 3]
   stp     x8, x9, [sp, #16 * 4]
   stp     x10, x11, [sp, #16 * 5]
   stp     x12, x13, [sp, #16 * 6]
   stp     x14, x15, [sp, #16 * 7]
   stp     x16, x17, [sp, #16 * 8]
   stp     x18, x19, [sp, #16 * 9]
   stp     x20, x21, [sp, #16 * 10]
   stp     x22, x23, [sp, #16 * 11]
   stp     x24, x25, [sp, #16 * 12]
   stp     x26, x27, [sp, #16 * 13]
   stp     x28, x29, [sp, #16 * 14]
   str     x30, [sp, #16 * 15]
   
   mrs     x22, elr_el1
   mrs     x23, spsr_el1
 
   stp	  x30, x22, [sp, #16 * 15] 
   str	  x23, [sp, #16 * 16]
.endm

.macro	kernel_exit

   ldp   x30, x22, [sp, #16 * 15]
   ldr   x23, [sp, #16 * 16] 

   msr   elr_el1, x22			
   msr   spsr_el1, x23

   ldp   x0, x1, [sp, #16 * 0]
   ldp   x2, x3, [sp, #16 * 1]
   ldp   x4, x5, [sp, #16 * 2]
   ldp   x6, x7, [sp, #16 * 3]
   ldp   x8, x9, [sp, #16 * 4]
   ldp   x10, x11, [sp, #16 * 5]
   ldp   x12, x13, [sp, #16 * 6]
   ldp   x14, x15, [sp, #16 * 7]
   ldp   x16, x17, [sp, #16 * 8]
   ldp   x18, x19, [sp, #16 * 9]
   ldp   x20, x21, [sp, #16 * 10]
   ldp   x22, x23, [sp, #16 * 11]
   ldp   x24, x25, [sp, #16 * 12]
   ldp   x26, x27, [sp, #16 * 13]
   ldp   x28, x29, [sp, #16 * 14]
   add   sp, sp, #16 * 17		
   eret
.endm

.macro handle_invalid_entry type
    kernel_entry
    mov    x0, #\type
    mrs    x1, esr_el1
    mrs    x2, elr_el1
    mov    x3, sp
    bl     show_invalid_entry_message
    b      err_hang
.endm

err_hang:
    b err_hang

.macro ventry label
.align 7
    b \label
.endm

.align 11
.globl vectors
vectors:
    ventry sync_invalid_el1t
    ventry irq_invalid_el1t
    ventry fiq_invalid_el1t
    ventry error_invalid_el1t
    
    ventry sync_invalid_el1h
    ventry irq_el1
    ventry fiq_invalid_el1h
    ventry error_invalid_el1h

    ventry sync_invalid_el0_64
    ventry irq_invalid_el0_64
    ventry fiq_invalid_el0_64
    ventry error_invalid_el0_64

    ventry sync_invalid_el0_32
    ventry irq_invalid_el0_32
    ventry fiq_invalid_el0_32
    ventry error_invalid_el0_32


 sync_invalid_el1t:
    handle_invalid_entry 0
 irq_invalid_el1t:
    handle_invalid_entry 1
 fiq_invalid_el1t:
    handle_invalid_entry 2
 error_invalid_el1t:
    handle_invalid_entry 3
    
 sync_invalid_el1h:
    handle_invalid_entry 4
 irq_invalid_el1h:
    handle_invalid_entry 5
 fiq_invalid_el1h:
    handle_invalid_entry 6
 error_invalid_el1h:
    handle_invalid_entry 7

 sync_invalid_el0_64:
    handle_invalid_entry 8
 irq_invalid_el0_64:
    handle_invalid_entry 9
 fiq_invalid_el0_64:
    handle_invalid_entry 10
 error_invalid_el0_64:
    handle_invalid_entry 11

 sync_invalid_el0_32:
    handle_invalid_entry 12
 irq_invalid_el0_32:
    handle_invalid_entry 13
 fiq_invalid_el0_32:
    handle_invalid_entry 14
 error_invalid_el0_32:
    handle_invalid_entry 15

irq_el1:
   kernel_entry
   bl handle_irq
   kernel_exit

//.globl ret_from_fork
//ret_from_fork:
//   //bl schedule_tail
//   mov x0, x20
//   mov x1, x21
//   blr x19
//   //bl exit

.globl cpu_switch_to
cpu_switch_to:
	mov	x8, x0
	mov	x9, sp
	stp	x19, x20, [x8], #16		// store callee-saved registers
	stp	x21, x22, [x8], #16
	stp	x23, x24, [x8], #16
	stp	x25, x26, [x8], #16
	stp	x27, x28, [x8], #16
	stp	x29, x9, [x8], #16
	str	x30, [x8]
.globl cpu_ctx_restore
cpu_ctx_restore:
	mov	x8, x1
	ldp	x19, x20, [x8], #16		// restore callee-saved registers
	ldp	x21, x22, [x8], #16
	ldp	x23, x24, [x8], #16
	ldp	x25, x26, [x8], #16
	ldp	x27, x28, [x8], #16
	ldp	x29, x9, [x8], #16
	ldr	x30, [x8]
	mov	sp, x9
	ret
