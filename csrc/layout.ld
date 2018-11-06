ENTRY(__start)

SECTIONS
{
  . = 0x40000000;
  .text : { 
    *(.text .text.*) 
  }
  .data : { *(.data) }
  .bss : { *(.bss) }
}