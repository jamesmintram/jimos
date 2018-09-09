.text

.global enter_virtual_addressing
enter_virtual_addressing:

  // load stack pointer (on 32bit)
  adrp	x2, __cpu0_stack_end
  add	  x2, x2, #:lo12:__cpu0_stack_end

  mov	  sp, x2

__clear_bss:
  ldr	x0, =__bss_start
  ldr	x1, =__bss_end

__clear:
  str	xzr, [x0]
  add	x0, x0, #1

  cmp x0, x1
	b.ls	__clear

__go_main:
  bl	kmain


.global start_userspace
start_userspace:
  //Move down to EL0
  // Status register for EL0
  mov x0, xzr
  //orr x0, x0, #(1 << 2)     // .M[3:2] = 0b100 -->  Return to EL1
  //orr x0, x0, #(1 << 6)     // FIQ masked
  //orr x0, x0, #(1 << 7)     // IRQ masked
  //orr x0, x0, #(1 << 8)     // SError (System Error) masked
  //orr x0, x0, #(1 << 9)     // Watchpoint, Breakpoint, and Software are masked
  msr spsr_el1, x0

  // Drop down to EL0
  adr     x0, entered_0
  msr     elr_el1, x0
  eret
entered_0:
  // b user_main

__hang:
	# wait forever in case of exit
	wfe
	b		__hang

.global exit
exit:
  // b __hang
  mov w0, 0x18
  mov x1, #0x20000
  add x1, x1, #0x26
  hlt #0xF000

.global call
call:
  stp	x29, x30, [sp, #-16]!
  mov	x29, sp

  mov x19, 0x100000
  blr	x19

  ldp	x29, x30, [sp], #16
  ret





# add section debug inf
// .size	__start_ram, . - __start_ram

_bss_segment:
  .dword	__bss_start
  .dword	__bss_dwords
