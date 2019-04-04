ENTRY(__start)

SECTIONS
{
  . = 0x10000;
  .text : { 
    *(.text .text.*) 
  }
  .data : { *(.data) }
  .bss : { *(.bss) }
}