#include "asm.h"
#include "struct_defs.h"

	.text



.macro	restore_frame el
  //x0 - points to the start of the TrapFrame struct

	ldr	x10, [x0, #(TF_ELR)]
  msr	elr_el1, x10

  mov x11, xzr
  mov x12, xzr
	ldp	w11, w12, [x0, #(TF_SPSR)]

	msr	spsr_el1, x11
	msr	esr_el1, x12

	ldp	x28, x29, [x0, #(TF_X + 28 * 8)]
	ldp	x26, x27, [x0, #(TF_X + 26 * 8)]
	ldp	x24, x25, [x0, #(TF_X + 24 * 8)]
	ldp	x22, x23, [x0, #(TF_X + 22 * 8)]
	ldp	x20, x21, [x0, #(TF_X + 20 * 8)]
	ldp	x16, x17, [x0, #(TF_X + 16 * 8)]
	ldp	x14, x15, [x0, #(TF_X + 14 * 8)]
	ldp	x12, x13, [x0, #(TF_X + 12 * 8)]
	ldp	x10, x11, [x0, #(TF_X + 10 * 8)]
	ldp	x8,  x9,  [x0, #(TF_X + 8  * 8)]
	ldp	x6,  x7,  [x0, #(TF_X + 6  * 8)]
	ldp	x4,  x5,  [x0, #(TF_X + 4  * 8)]
	ldp	x2,  x3,  [x0, #(TF_X + 2  * 8)]

  ldp	x18, x30, [x0, #(TF_SP)]

.if \el == 0
  msr sp_el0, x18
.else
  mov sp, x18
.endif

  ldp	x18, x19, [x0, #(TF_X + 18 * 8)]
  ldp	x0,  x1,  [x0, #(TF_X + 0  * 8)]
.endm





.macro	restore_registers_initial el
  //x0 - points to the start of the TrapFrame struct

	ldr	x10, [x0, #(TF_ELR)]
  msr	elr_el1, x10

  mov x11, xzr
  mov x12, xzr
	ldp	w11, w12, [x0, #(TF_SPSR)]

	msr	spsr_el1, x11
	msr	esr_el1, x12

	ldp	x28, x29, [x0, #(TF_X + 28 * 8)]
	ldp	x26, x27, [x0, #(TF_X + 26 * 8)]
	ldp	x24, x25, [x0, #(TF_X + 24 * 8)]
	ldp	x22, x23, [x0, #(TF_X + 22 * 8)]
	ldp	x20, x21, [x0, #(TF_X + 20 * 8)]
	ldp	x16, x17, [x0, #(TF_X + 16 * 8)]
	ldp	x14, x15, [x0, #(TF_X + 14 * 8)]
	ldp	x12, x13, [x0, #(TF_X + 12 * 8)]
	ldp	x10, x11, [x0, #(TF_X + 10 * 8)]
	ldp	x8,  x9,  [x0, #(TF_X + 8  * 8)]
	ldp	x6,  x7,  [x0, #(TF_X + 6  * 8)]
	ldp	x4,  x5,  [x0, #(TF_X + 4  * 8)]
	ldp	x2,  x3,  [x0, #(TF_X + 2  * 8)]

  ldp	x18, x30, [x0, #(TF_SP)]

.if \el == 0
  msr sp_el0, x18
.else
  mov sp, x18
.endif
	

  ldp	x18, x19, [x0, #(TF_X + 18 * 8)]
  ldp	x0,  x1,  [x0, #(TF_X + 0  * 8)]
.endm


.macro	save_registers el

  /*
    --------------------------- SP + TF_SIZE + 16
    x30     | Back trace
    x29     |
    --------------------------- SP + TF_SIZE
    TrapFrame
    --------------------------- SP

    X29 Holds SP + TF_SIZE (ie the Back trace)
  */

	sub	sp, sp, #(TF_SIZE + 16)
	stp	x29, x30, [sp, #(TF_SIZE)]

	stp	x28, x29, [sp, #(TF_X + 28 * 8)]
	stp	x26, x27, [sp, #(TF_X + 26 * 8)]
	stp	x24, x25, [sp, #(TF_X + 24 * 8)]
	stp	x22, x23, [sp, #(TF_X + 22 * 8)]
	stp	x20, x21, [sp, #(TF_X + 20 * 8)]
	stp	x18, x19, [sp, #(TF_X + 18 * 8)]
	stp	x16, x17, [sp, #(TF_X + 16 * 8)]
	stp	x14, x15, [sp, #(TF_X + 14 * 8)]
	stp	x12, x13, [sp, #(TF_X + 12 * 8)]
	stp	x10, x11, [sp, #(TF_X + 10 * 8)]
	stp	x8,  x9,  [sp, #(TF_X + 8  * 8)]
	stp	x6,  x7,  [sp, #(TF_X + 6  * 8)]
	stp	x4,  x5,  [sp, #(TF_X + 4  * 8)]
	stp	x2,  x3,  [sp, #(TF_X + 2  * 8)]
	stp	x0,  x1,  [sp, #(TF_X + 0  * 8)]

	mrs	x10, elr_el1
	mrs	x11, spsr_el1
	mrs	x12, esr_el1

	str	x10, [sp, #(TF_ELR)]
	stp	w11, w12, [sp, #(TF_SPSR)]

  mov x18, sp
  stp	x18, x30, [sp, #(TF_SP)] //Assembler did not like lr so x30

  add	x29, sp, #(TF_SIZE) //Keeps a valid BackTrace
.endm

.macro	restore_registers el
  //x0 - points to the start of the TrapFrame struct

  sub	x0, x0, #(TF_SIZE)
	ldr	x10, [x0, #(TF_ELR)]
  msr	elr_el1, x10

  mov x11, xzr
  mov x12, xzr
	ldp	w11, w12, [x0, #(TF_SPSR)]

	msr	spsr_el1, x11
	msr	esr_el1, x12

	ldp	x28, x29, [x0, #(TF_X + 28 * 8)]
	ldp	x26, x27, [x0, #(TF_X + 26 * 8)]
	ldp	x24, x25, [x0, #(TF_X + 24 * 8)]
	ldp	x22, x23, [x0, #(TF_X + 22 * 8)]
	ldp	x20, x21, [x0, #(TF_X + 20 * 8)]
	ldp	x16, x17, [x0, #(TF_X + 16 * 8)]
	ldp	x14, x15, [x0, #(TF_X + 14 * 8)]
	ldp	x12, x13, [x0, #(TF_X + 12 * 8)]
	ldp	x10, x11, [x0, #(TF_X + 10 * 8)]
	ldp	x8,  x9,  [x0, #(TF_X + 8  * 8)]
	ldp	x6,  x7,  [x0, #(TF_X + 6  * 8)]
	ldp	x4,  x5,  [x0, #(TF_X + 4  * 8)]
	ldp	x2,  x3,  [x0, #(TF_X + 2  * 8)]

  ldp	x18, x19, [x0, #(TF_X + 18 * 8)]
  ldp	x0,  x1,  [x0, #(TF_X + 0  * 8)]

  add	sp, sp, #(TF_SIZE + 16)
.endm


//-------------------------------------------------------


.macro	ctx_switch_save_registers el
	sub	sp, sp, #(TF_SIZE)
	
  //TODO: User a different darta struct for ctx_switch
  str	x30, [sp, #(TF_LR)]

	stp	x28, x29, [sp, #(TF_X + 28 * 8)]
	stp	x26, x27, [sp, #(TF_X + 26 * 8)]
	stp	x24, x25, [sp, #(TF_X + 24 * 8)]
  stp	x22, x23, [sp, #(TF_X + 22 * 8)]
	stp	x20, x21, [sp, #(TF_X + 20 * 8)]
	stp	x18, x19, [sp, #(TF_X + 18 * 8)]
	stp	x16, x17, [sp, #(TF_X + 16 * 8)]
	stp	x14, x15, [sp, #(TF_X + 14 * 8)]
	stp	x12, x13, [sp, #(TF_X + 12 * 8)]
	stp	x10, x11, [sp, #(TF_X + 10 * 8)]
	stp	x8,  x9,  [sp, #(TF_X + 8  * 8)]
	stp	x6,  x7,  [sp, #(TF_X + 6  * 8)]
	stp	x4,  x5,  [sp, #(TF_X + 4  * 8)]
	stp	x2,  x3,  [sp, #(TF_X + 2  * 8)]
	stp	x0,  x1,  [sp, #(TF_X + 0  * 8)]

  //TODO: Store the ttib
  //mrs  x19, tpidr_el0
  //mrs  x20, tpidrro_el0
  //stp x19, x20, [sp, #(TF_X + 0 * 8)] //TODO: NEW STRUCT!
.endm

.macro	ctx_switch_restore_registers el
  // Requires the SP already be set

  //ldp	x19, x20, [sp, #(TF_X + 0 * 8)] //TODO: NEW STRUCT!
  //msr  tpidr_el0, x19
  //msr  tpidrro_el0, x20

  ldr	x30, [sp, #(TF_LR)]

	ldp	x28, x29, [sp, #(TF_X + 28 * 8)]
	ldp	x26, x27, [sp, #(TF_X + 26 * 8)]
	ldp	x24, x25, [sp, #(TF_X + 24 * 8)]
	ldp	x22, x23, [sp, #(TF_X + 22 * 8)]
	ldp	x20, x21, [sp, #(TF_X + 20 * 8)]
	ldp	x16, x17, [sp, #(TF_X + 16 * 8)]
	ldp	x14, x15, [sp, #(TF_X + 14 * 8)]
	ldp	x12, x13, [sp, #(TF_X + 12 * 8)]
	ldp	x10, x11, [sp, #(TF_X + 10 * 8)]
	ldp	x8,  x9,  [sp, #(TF_X + 8  * 8)]
	ldp	x6,  x7,  [sp, #(TF_X + 6  * 8)]
	ldp	x4,  x5,  [sp, #(TF_X + 4  * 8)]
	ldp	x2,  x3,  [sp, #(TF_X + 2  * 8)]
  ldp	x0,  x1,  [sp, #(TF_X + 0  * 8)]

  add	sp, sp, #(TF_SIZE)
.endm

.global	_initial_thread_start
_initial_thread_start:
  // X0 = pointer to the next thread block

  // Switch the thread pointer to the next thread block
  msr tpidr_el1, x0
  //TODO: Memory or instruction barrier required?

  // Switch page tables (TODO)

  // Load the next thread's SP
  ldr x18, [x0, TB_SP]
  mov sp, x18

  // Load system registers (TODO: This should move)
	ldr	x10, [x0, #(TF_ELR)]
  msr	elr_el1, x10

  mov x11, xzr
  mov x12, xzr
	ldp	w11, w12, [x0, #(TF_SPSR)]

	msr	spsr_el1, x11
	msr	esr_el1, x12

  ctx_switch_restore_registers
  ret


.global _ctx_switch
_ctx_switch:
  // X0 = pointer to current thread block
  // X1 = pointer to the next thread block
  //NOTE: X0 and X1 must not be modified by save_registers
  
  clrex
  
  ctx_switch_save_registers
  // X0 contains the new SP for the thread we just switched away from
  
  // Store the thread's SP value in it's thread block  
  mov x18, sp
  str x18, [x0, TB_SP]

  // Switch the thread pointer to the next thread block
  msr tpidr_el1, x1
  
  // Switch page tables

  // Load the next thread's SP
  ldr x18, [x1, TB_SP]
  mov sp, x18

  //Finally restore the registers
  ctx_switch_restore_registers

  ret


//TODO: This is currently written for the first time
//      a process enters EL0
  .global _enter_userspace
_enter_userspace:

  //MSR SPSel, #0
  //mov x10, 0xB00B
  //msr elr_el1, x10

  //Enter EL0
  //---------

  //ensure elr will be "restored" to the entered_0
  //adr x10, entered_0
  //str	x10, [x0, #(TF_ELR)]

  restore_frame 0

  eret

entered_0:


  //If we ever reach here, then a process has terminated
  //therefore we need to make a syscall
  b exit

ENTRY(handle_err_sync)
  b exit
END(handle_err_sync)

ENTRY(handle_el1h_sync)

  MSR SPSel, #1 // SP_ELn

  save_registers 1
	
  //TODO: Fix this broken return
	mov	x0, sp
	bl	do_el1h_sync

	mov x0, sp
  restore_registers 1
  
  //b exit
	eret
END(handle_el1h_sync)

ENTRY(handle_el0_sync)
  // HACK for making SP work (For now TODO: Fix)
  MSR SPSel, #1 // SP_ELn

  save_registers 0
	//ldr	x0, [x18, #PC_CURTHREAD]
	mov	x0, sp
	bl	do_el0_sync
	restore_registers 0
  //b exit
	eret
END(handle_el0_sync)

.macro vempty
  .align 7
  mov     x0, #3
  mrs     x1, esr_el1
  mrs     x2, elr_el1
  mrs     x3, spsr_el1
  mrs     x4, far_el1
  b       exit
.endm

.macro vector name
  .align 7
	b	handle_\name
.endm

	.text
	.align 12

  .align 11
	.global _vectors
_vectors:
  //TODO: Eventually we should not use this

  vempty  //Synchronous EL1t
  vempty  //IRQ EL1t
  vempty  //FIQ EL1t
  vector err_sync  //Error EL1t

  //TODO: Eventually we should use this

  vector el1h_sync  //Synchronous EL1h
  vempty  //IRQ EL1h
  vempty  //FIQ EL1h
  vector err_sync  //Error EL1h

  vector el0_sync  //Synchronous 64bit EL0
  vempty  //IRQ 64bit EL0
  vempty  //FIQ 64bit EL0
  vector err_sync  //Error 64bit EL0

  vempty  //Synchronous 32bit EL0
  vempty  //IRQ 32bit EL0
  vempty  //FIQ 32bit EL0
  vempty  //Error 32bit EL0


  // These are temporary for now (To be replaced by the above)
  //--------------------------------------------------


  .align 11
  .global _vectors_disable
_vectors_disable:
  //-------------------------------------
  // Current EL with SP0
  //-------------------------------------

  // synchronous
  .align  7
  mov     x0, #0
  mrs     x1, esr_el1
  mrs     x2, elr_el1
  mrs     x3, spsr_el1
  mrs     x4, far_el1
  b       exit

  // IRQ
  .align  7
  mov     x0, #1
  mrs     x1, esr_el1
  mrs     x2, elr_el1
  mrs     x3, spsr_el1
  mrs     x4, far_el1
  b       exit

  // FIQ
  .align  7
  mov     x0, #2
  mrs     x1, esr_el1
  mrs     x2, elr_el1
  mrs     x3, spsr_el1
  mrs     x4, far_el1
  b       exit

  // SError
  .align  7
  mov     x0, #3
  mrs     x1, esr_el1
  mrs     x2, elr_el1
  mrs     x3, spsr_el1
  mrs     x4, far_el1
  b       exit

  //-------------------------------------
  // Current EL with SPx
  //-------------------------------------

  // synchronous - from SVC
  .align  7
  mov     x0, #0
  mrs     x1, esr_el1
  mrs     x2, elr_el1
  mrs     x3, spsr_el1
  mrs     x4, far_el1
  b       exit

  // IRQ
  .align  7
  mov     x0, #1
  mrs     x1, esr_el1
  mrs     x2, elr_el1
  mrs     x3, spsr_el1
  mrs     x4, far_el1
  b       exit

  // FIQ
  .align  7
  mov     x0, #2
  mrs     x1, esr_el1
  mrs     x2, elr_el1
  mrs     x3, spsr_el1
  mrs     x4, far_el1
  b       exit

  // SError
  .align  7
  mov     x0, #3
  mrs     x1, esr_el1
  mrs     x2, elr_el1
  mrs     x3, spsr_el1
  mrs     x4, far_el1
  b       exit


  //-------------------------------------
  // Lower EL using AARCH64
  //-------------------------------------

  // synchronous - from SVC
  .align  7

  //TODO: Support for context kernel stack?
  MSR SPSel, #0

  //TODO: Clean up this "syscall" specific code
  sub	sp, sp, #0x20
  str	x30, [sp, #16]

  bl int_syscall

  ldr	x30, [sp, #16]
  add	sp, sp, #0x20

  // mov     x0, #0
  // mrs     x1, esr_el1
  // mrs     x2, elr_el1
  // mrs     x3, spsr_el1
  // mrs     x4, far_el1


  eret

  // IRQ
  .align  7
  mov     x0, #1
  mrs     x1, esr_el1
  mrs     x2, elr_el1
  mrs     x3, spsr_el1
  mrs     x4, far_el1
  b       exit

  // FIQ
  .align  7
  mov     x0, #2
  mrs     x1, esr_el1
  mrs     x2, elr_el1
  mrs     x3, spsr_el1
  mrs     x4, far_el1
  b       exit

  // SError
  .align  7
  mov     x0, #3
  mrs     x1, esr_el1
  mrs     x2, elr_el1
  mrs     x3, spsr_el1
  mrs     x4, far_el1
  b       exit


//////////////////////////////////////
/// Dead code


.macro	save_registers_bsd el
.if \el == 1
	mov	x18, sp
	sub	sp, sp, #128
.endif
	sub	sp, sp, #(TF_SIZE + 16)
	stp	x29, x30, [sp, #(TF_SIZE)]
	stp	x28, x29, [sp, #(TF_X + 28 * 8)]
	stp	x26, x27, [sp, #(TF_X + 26 * 8)]
	stp	x24, x25, [sp, #(TF_X + 24 * 8)]
	stp	x22, x23, [sp, #(TF_X + 22 * 8)]
	stp	x20, x21, [sp, #(TF_X + 20 * 8)]
	stp	x18, x19, [sp, #(TF_X + 18 * 8)]
	stp	x16, x17, [sp, #(TF_X + 16 * 8)]
	stp	x14, x15, [sp, #(TF_X + 14 * 8)]
	stp	x12, x13, [sp, #(TF_X + 12 * 8)]
	stp	x10, x11, [sp, #(TF_X + 10 * 8)]
	stp	x8,  x9,  [sp, #(TF_X + 8  * 8)]
	stp	x6,  x7,  [sp, #(TF_X + 6  * 8)]
	stp	x4,  x5,  [sp, #(TF_X + 4  * 8)]
	stp	x2,  x3,  [sp, #(TF_X + 2  * 8)]
	stp	x0,  x1,  [sp, #(TF_X + 0  * 8)]
	mrs	x10, elr_el1
	mrs	x11, spsr_el1
	mrs	x12, esr_el1

.if \el == 0
  mrs	x18, sp_el0
.endif
	str	x10, [sp, #(TF_ELR)]
	stp	w11, w12, [sp, #(TF_SPSR)]
	stp	x18, x30, [sp, #(TF_SP)] //Assembler did not like lr so x30
	mrs	x18, tpidr_el1
	add	x29, sp, #(TF_SIZE)
.endm



.macro	restore_registers_bsd el
  //TODO: Disable interrupts

	ldp	x18,  x30, [sp, #(TF_SP)] //Assembler did not like lr so x30
	ldp	x10, x11, [sp, #(TF_ELR)]
.if \el == 0
	mov x18, 0x40000000
  orr x18, x18, 0x10
  msr	sp_el0, x18

.endif
	msr	spsr_el1, x11
	msr	elr_el1, x10
	ldp	x0,  x1,  [sp, #(TF_X + 0  * 8)]
	ldp	x2,  x3,  [sp, #(TF_X + 2  * 8)]
	ldp	x4,  x5,  [sp, #(TF_X + 4  * 8)]
	ldp	x6,  x7,  [sp, #(TF_X + 6  * 8)]
	ldp	x8,  x9,  [sp, #(TF_X + 8  * 8)]
	ldp	x10, x11, [sp, #(TF_X + 10 * 8)]
	ldp	x12, x13, [sp, #(TF_X + 12 * 8)]
	ldp	x14, x15, [sp, #(TF_X + 14 * 8)]
	ldp	x16, x17, [sp, #(TF_X + 16 * 8)]
.if \el == 0
	/*
	 * We only restore the callee saved registers when returning to
	 * userland as they may have been updated by a system call or signal.
	 */
	ldp	x18, x19, [sp, #(TF_X + 18 * 8)]
	ldp	x20, x21, [sp, #(TF_X + 20 * 8)]
	ldp	x22, x23, [sp, #(TF_X + 22 * 8)]
	ldp	x24, x25, [sp, #(TF_X + 24 * 8)]
	ldp	x26, x27, [sp, #(TF_X + 26 * 8)]
	ldp	x28, x29, [sp, #(TF_X + 28 * 8)]
.else
	ldr	     x29, [sp, #(TF_X + 29 * 8)]
.endif
.if \el == 0
	add	sp, sp, #(TF_SIZE + 16)
.else
	mov	sp, x18
	mrs	x18, tpidr_el1
.endif
.endm