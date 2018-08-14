

#define TABLE_SHIFT       9   //9 bits of address space per table (512 entries)
#define PAGE_SHIFT        12  //4096 bytes per page - lower 12 bits
#define SECTION_SHIFT     (PAGE_SHIFT + TABLE_SHIFT)  //Bits remaining for the offset within a 2MB section (21 for 2MB, 12 for 4k)
#define SECTION_SIZE      (1 << SECTION_SHIFT)  //21 Bits of address = 2MB

#define PAGE_SIZE         0x1000
#define TABLE_SIZE        0x1000

#define PTRS_PER_TABLE    (1 << 9)

#define MM_ACCESS			(0x1 << 10)
#define MM_ACCESS_PERMISSION	(0x01 << 6)

#define MM_BLOCK_DESCRIPTOR  0b01
#define MM_TABLE_DESCRIPTOR  0b11


#define VA_START          0xFFFF000000000000

#define PHYSICAL_RAM      0x20000000 //512mb


#define TCR_T0SZ			(64 - 48)
#define TCR_T1SZ			((64 - 48) << 16)
#define TCR_TG0_4K			(0 << 14)
#define TCR_TG1_4K			(2 << 30)
#define TCR_VALUE	(TCR_T0SZ | TCR_T1SZ | TCR_TG0_4K | TCR_TG1_4K)