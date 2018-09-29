#include "consts.h"


.section .traps, "ax"
.global __start
__start:
  b		__start_ram

.text
.type __start_ram, %function
__start_ram:
  // read processor number, move slave processors to an infinite loop
  mrs	  x7, mpidr_el1
  and	  x7, x7, #3
  cbz	  x7, __start_master
  b		  __hang

__start_master:
  //Hacky - but we use the Page table space as a stack for now
  adrp	x2, boot_end
  add	  x2, x2, #:lo12:boot_end

  mov	  sp, x2
  
  bl enable_jtag

//--------------------------------------------------------
// Drop down to EL1

  mrs x0, CurrentEL
  and x0, x0, #12

  cmp x0, #12
  bne entered_2

 // This will not run on RPI3 (only in Qemu for me)
 entered_3:
   //Disable MMU
   msr   sctlr_el2, xzr

   mov   x0, xzr
   orr   x0, x0, #(1 << 10)         // .RW = 0b1  -->  EL2 is AArch64
   orr   x0, x0, #(1 << 0)          // .NS = 0b1  -->  Non-secure state
   msr   scr_el3, x0

   mov   x0, xzr
   orr   x0, x0, #(1 << 0)   // .M[0] = 0b1     -->  Use EL2's dedicated stack pointer
   orr   x0, x0, #(1 << 3)   // .M[3:2] = 0b100 -->  Return to EL2
   orr   x0, x0, #(1 << 6)   // FIQ masked
   orr   x0, x0, #(1 << 7)   // IRQ masked
   orr   x0, x0, #(1 << 8)   // SError (System Error) masked
   orr   x0, x0, #(1 << 9)   // Watchpoint, Breakpoint, and Software are masked

   msr   spsr_el3, x0

   adr  x0, entered_2
   msr  elr_el3, x0

   eret

 entered_2:
  // Check if we are already in EL1 - if so, jump over
  mrs   x0, CurrentEL
  and   x0, x0, #12
  cmp   x0, #4
  beq   entered_1

  // Enable AArch64
  mov   x0, xzr
  orr   x0, x0, #(1 << 31)  // .RW   -> EL1 is AArch64
  orr   x0, x0, #(1 << 1)   // .SWIO -> hardwired on Pi3
  msr   hcr_el2, x0
  mrs   x0, hcr_el2

  // Enable CNTP
  mrs   x0, cnthctl_el2
  orr   x0, x0, #(1 << 1)   // .EL1PCEN  -> dont trap physical timer registers
  orr   x0, x0, #(1 << 0)   // .EL1PCTEN -> dont trap physical counter registers
  msr   cnthctl_el2, x0

  msr   cntvoff_el2, xzr    //Disable Counter-timer Virtual Offset register

    //FIGURE OUT WHAT THIS STUFF DOES...
    //ALSO - SHOULD ONLY BE IN EL0? (RUST FLAGS TO DISABLE FPU)
    // disable coprocessor traps
    //mov     x0, #0x33FF
    //msr     cptr_el2, xzr
    //msr     hstr_el2, xzr

  //NOTE: Disabled FP instructions for EL1 code
  // Enable SIMD and FP instructions 
  // TODO: Move this to context switching 
  //    - we do need this for EL0 
  //    - we can also turn it on "just in time" using an exception - then context switching can be faster (not restoring all thsoe registers)
  // mov     x0, #(3 << 20)  //.FPEN[21:20] = 0b11 --> No instructions to be trapped
  // msr     cpacr_el1, x0

  // Status register for EL1
  mov x0, xzr
  orr x0, x0, #(1 << 2)     // .M[3:2] = 0b100 -->  Return to EL1
  orr x0, x0, #(1 << 6)     // FIQ masked
  orr x0, x0, #(1 << 7)     // IRQ masked
  orr x0, x0, #(1 << 8)     // SError (System Error) masked
  orr x0, x0, #(1 << 9)     // Watchpoint, Breakpoint, and Software are masked

  msr spsr_el2, x0

  // Drop down to EL1
  adr     x0, entered_1
  msr     elr_el2, x0

  eret

//--------------------------------------------------------
//  This is the point where our kernel starts
//--------------------------------------------------------
entered_1:

  // First thing to do is enable the MMU

  // Memset all of it to 0

  adrp x20, __page_tables_start
  adrp x21, __page_tables_size

  lsr x21, x21, #4

  // Set all page table memory to zero
1:sub x21, x21, #2

  stp xzr, xzr, [x20]
  add x20, x20, #16

  cbnz x21, 1b

  // Page table descriptor
  // [63 Upper Attribs 48|47 Address  12|11 Lower Attribs 2|1 Block/Table bit 1|0 Valid bit 0]


  //Setup first entry of PGD
  //------------------------
  adrp x21, __page_tables_start

  // Address of next PT = __page_tables_start + sizeof(PGD) = 4096
  adrp x20, __page_tables_start
  add x20, x20, TABLE_SIZE

	//Shift into address field - dont need this coz of 4096 page size!
  //lsl x20, x20, PAGE_SHIFT

  // Set lower attribs
  // Set Block/Table
  orr x20, x20, MM_TABLE_DESCRIPTOR

	// Store at __page_tables_start[0]
	str x20, [x21]

  //Setup first entry of PUD
  //------------------------
  adrp x21, __page_tables_start
  add x21, x21, TABLE_SIZE

  adrp x20, __page_tables_start
  add x20, x20, TABLE_SIZE * 2

  orr x20, x20, MM_TABLE_DESCRIPTOR
  str x20, [x21]
  //-----------------------------



  // Now we setup the L3 entries
	//-----------------------------
  adrp x20, __page_tables_start     // L3 Base pointer
  add x20, x20, TABLE_SIZE * 2
 
  adrp x21, __page_tables_start     // L4[i] Table address
  add x21, x21, TABLE_SIZE * 3

	orr x21, x21, MM_TABLE_DESCRIPTOR

  mov x22, #0   // Entry counter
  mov x23, #512

1: str x21, [x20, x22, lsl #3]      // LSL makes each entry 1 << 3 or 8 bytes
  add x21, x21, TABLE_SIZE          // inc physical address
  add x22, x22, #1                  // inc index

  cmp x22, x23                      // are we at end?
  b.ls 1b
//-----------------------------



// Now we setup the L4 entries
//-----------------------------

  adrp x20, __page_tables_start     //L4[0] Base pointer
  add x20, x20, TABLE_SIZE * 3

  // Current Table Entry :: Physical address + flags
  // ldr x21, =kernel_physical_start
  mov x21, #0                       //Identity map the who range of memory

	orr x21, x21, MM_ACCESS
	orr x21, x21, MM_TABLE_DESCRIPTOR
  orr x21, x21, PT_ISH


  ldr x23, =0x3EFFFFFF              // End of physical mapping
  mov x22, #0                       // L4[i] Entry counter

1: str x21, [x20, x22, lsl #3]      // LSL makes each entry 1 << 3 or 8 bytes
   add x21, x21, SECTION_SIZE       // inc physical address
   add x22, x22, #1                 // inc index

   cmp x21, x23
   b.ls 1b

  // Change the flags for the last 16MB
  ldr x21, =0x3F000000
	orr x21, x21, MM_ACCESS
	orr x21, x21, MM_TABLE_DESCRIPTOR
  orr x21, x21, PT_OSH
  orr x21, x21, PT_DEV
  //TODO: Setup MAIR reference

  ldr x23, =0x3FFFFFFF          // End of MMIO

1: str x21, [x20, x22, lsl #3]  // LSL makes each entry 1 << 3 or 8 bytes
  add x21, x21, SECTION_SIZE    // inc physical address
  add x22, x22, #1              // inc index

  cmp x21, x23                  // are we at end?
  b.le 1b
//-----------------------------



  //Setup MAIR registers
  mov x20, xzr

  //MT_DEVICE_NORMAL
  //mov x21, 0b01000100       // 0x44
  mov x21, 0xFF       
  orr x20, x20, x21         // Mair IDX 0

  mov x21, 0b00000100       // 0x04
  orr x20, x20, x21, lsl #8 // Mair IDX 1

	msr mair_el1, x20


  //Setup the page table ptr
	adrp x20, __page_tables_start
	msr ttbr1_el1, x20
  msr ttbr0_el1, x20

  mov x20, xzr

  //Autodetect and set IPS
  mrs x18, id_aa64mmfr0_el1
  and x18, x18, 0xF
  lsl x18, x18, 32
  orr x20, x20, x18

  //orr x20, x20, 0b10 << 30
  orr x20, x20, 0b11 << 28
  orr x20, x20, 0b01 << 26
  orr x20, x20, 0b01 << 24

  mov x18, #25 << 16    //25 = 0001 1001
  //orr x20, x20, x18

  orr x20, x20, 0b11 << 12
  orr x20, x20, 0b01 << 10
  orr x20, x20, 0b01 << 8

  mov x18, #25 << 0
  //orr x20, x20, x18

  //adrp x20, __page_tables_start
	msr tcr_el1, x20

  //0xb5193519
  //0000 0000 1011 0101 0001 1001 0011 0101 0001 1001

  // TEMP:
  //mov w0, 0x18
	//mov x1, #0x20000
	//add x1, x1, #0x26
	//hlt #0xF000

  // b __hang


  // set up exception handlers in VA space
	ldr     x0, =_vectors
	msr     vbar_el1, x0

jump_prep:
  // Now load up the code to jump to
  ldr x0, =enter_virtual_addressing

  mrs x2, mair_el1

	dsb ish
  isb


  mov x2, xzr
  orr x2, x2, #1 << 25
  orr x2, x2, #1 << 24
  orr x2, x2, #1 << 19
  orr x2, x2, #1 << 12
  
  orr x2, x2, #1 << 4
  orr x2, x2, #1 << 3
  orr x2, x2, #1 << 2
  orr x2, x2, #1 << 1
  

  ldr x1, =0xC00800
  bic x1, x1, x2
  orr x1, x1, #1 << 0

  msr sctlr_el1, x1

  isb

  //mov w0, 0x18
	//mov x1, #0x20000
	//add x1, x1, #0x26
	//hlt #0xF000



  	//ldr x0,  =(VA_START | 0x00200000)
	  //ldr x0, [x0]

  br x0


  // TODO: update the system register
  // TODO: ISB
  // TODO: b x0

__hang:
  # wait forever in case of exit
	wfe
	b		__hang



//https://github.com/s-matyukevich/raspberry-pi-os/blob/master/docs/lesson06/rpi-os.md
//https://github.com/s-matyukevich/raspberry-pi-os/blob/master/src/lesson06/src/boot.S#L92
