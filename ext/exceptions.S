
.macro vempty
  .align 7
  brk 0xfff
  1:   b 1b
.endm

.macro vector name
  .align 7
.endm


	.text
	.align 12

  .align 11
	.global el0_vectors
el0_vectors:

  vempty  //Synchronous EL1t
  vempty  //IRQ EL1t
  vempty  //FIQ EL1t
  vempty  //Error EL1t

  vempty  //Synchronous EL1h
  vempty  //IRQ EL1h
  vempty  //FIQ EL1h
  vempty  //Error EL1h

  vempty  //Synchronous 64bit EL0
  vempty  //IRQ 64bit EL0
  vempty  //FIQ 64bit EL0
  vempty  //Error 64bit EL0

  vempty  //Synchronous 32bit EL0
  vempty  //IRQ 32bit EL0
  vempty  //FIQ 32bit EL0
  vempty  //Error 32bit EL0


  // These are temporary for now (To be replaced by the above)
  //--------------------------------------------------


  .align 11
  .global _vectors
_vectors:
  //-------------------------------------
  // Current EL with SP0
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
