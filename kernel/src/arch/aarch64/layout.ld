MEMORY {
  /*SRAM (rwx) : ORIGIN = 0, LENGTH = 32M*/
}

SECTIONS {
  kernel_va_base = 0xFFFF000000000000;
  kernel_page_size = 4K; 

  kps_less1 = kernel_page_size -1;
  kernel_physical_start = (boot_end + kps_less1) & ~(kps_less1);
  
  kernel_first_page = kernel_va_base + kernel_physical_start;

  /* Seems to give me what I need*/
  /*kernel_end = __kernel_stack_end;*/
  bss_phys_end = LOADADDR(.bss) + SIZEOF(.bss);
  bss_end = LOADADDR(.bss) + SIZEOF(.bss);

  __page_tables_size = ((3 + 512) * 0x1000);
  
  __page_tables_phys_start = bss_phys_end;
  __page_tables_phys_end = __page_tables_phys_start + __page_tables_size;
  
  __page_tables_start = bss_end;
  __page_tables_end = __page_tables_start + __page_tables_size;

  __heap_phys_start = __page_tables_phys_end;
  __heap_start = __page_tables_end;

  __boot_stack = __heap_phys_start + 0x100000;

  /*-----------------------------------------------*/
  /* This is the bootstrap
  /*------------------------------------------------*/

  .boot.text 0x0 : {
        KEEP (*(.traps))
        . = 0x80000;  /* Space for command line.  */

        KEEP(build/head.o (.text))
	      KEEP(build/jtag.o (.text))
  }

  .boot.data : {
    build/head.o (.data)
    build/jtag.o (.data)
  }

  .boot.bss  (NOLOAD) : {
    build/head.o (.bss)
    build/jtag.o (.data)
  }

  boot_end = .;
}

/*-----------------------------------------------*/
/* This is the start of the VA Kernel
/*------------------------------------------------*/

SECTIONS {
  .text kernel_first_page : AT (kernel_physical_start) {
     *(.text .text.* .gnu.linkonce.t*)
     /*Below keeps symbols from being stripped*/
     /* KEEP (*(*sys_*)) */
  }

  .rodata : {
    *(.rodata .rodata.* .gnu.linkonce.r*)
  }

  .data : {
    *(.data .data.* .gnu.linkonce.d*)
  }

  .bss  (NOLOAD) : {
    __bss_start = ALIGN(0x10);
    *(.bss .bss.*)
    *(COMMON)

    __bss_end = ALIGN(0x10);

    __kernel_stack_guard = ALIGN(kernel_page_size);
    . += kernel_page_size;

    /*TODO: Shouldn't really be sticking the stack here*/
    __kernel_stack_start = ALIGN(kernel_page_size);
    . += 0x100000;
    . = ALIGN(kernel_page_size);
    __kernel_stack_end = .;
  }

   __bss_dwords = (__bss_end - __bss_start) >> 3;


  /*
    Kernel page tables here
  */

  /*
    Heap starts here
  */


  /**
   * DWARF debug sections.
   *
   * Symbols in the DWARF debugging sections are relative to the beginning of
   * the section so we begin them at 0.
  **/

  /* DWARF 1.1 and DWARF 2 */
  .debug_aranges  0 : { *(.debug_aranges) }
  .debug_pubnames 0 : { *(.debug_pubnames) }

  /* DWARF 2 */
  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }
  .debug_abbrev   0 : { *(.debug_abbrev) }
  .debug_line     0 : { *(.debug_line) }
  .debug_frame    0 : { *(.debug_frame) }
  .debug_str      0 : { *(.debug_str) }
  .debug_loc      0 : { *(.debug_loc) }
  .debug_macinfo  0 : { *(.debug_macinfo) }

  /* DWARF 3 */
  .debug_pubtypes 0 : { *(.debug_pubtypes) }
  .debug_ranges   0 : { *(.debug_ranges) }

  .ARM.attributes 0 : { KEEP (*(.ARM.attributes)) KEEP (*(.gnu.attributes)) }
  .note.gnu.arm.ident 0 : { KEEP (*(.note.gnu.arm.ident)) }

  .ARM.extab : { *(.ARM.extab* .gnu.linkonce.armextab.*) }
  PROVIDE_HIDDEN (__exidx_start = .);
  .ARM.exidx : { *(.ARM.exidx* .gnu.linkonce.armexidx.*) }
  PROVIDE_HIDDEN (__exidx_end = .);

  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }
}
