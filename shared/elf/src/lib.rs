#![no_std]

use core::mem;
use core::slice;

#[repr(C)]
#[derive(Debug)]
pub struct ElfProgramHeaderTable
{
    seg_type: u32,      // 1 - Loadable segment
    flags: u32,         // 0x1 - Execute, 0x2 - Write, 0x4 - Read
    offset: u64,        // Offset in file
    vaddr: usize,       // Virtual address in memory
    paddr: usize,
    file_size: usize,   //Size of segment in file
    mem_size: u64,      //Size of segment in memory
    align: u64,         //Alignment of segment
}

#[repr(C)]
#[derive(Debug)]
pub struct ElfSectionHeader 
{
    name: u32,          //Section name offset
    pub section_type: u32,  //Section type
    flags: u64,         //Section attributes
    pub addr: usize,        //Virtual address in memory
    pub file_offset: usize, //Offset in file
    pub size: usize,        //Size of section
    link: u32,          //Link to other section
    info: u32,          //Misc information
    addalign: u64,      //Address alignment boundary
    entsize: u64,       //Size of entries, if section ahs table
}

#[repr(C)]
#[derive(Debug)]
pub struct EldHeaderIdent
{
    magic: [u8;4],      // 127, 69, 76, 70 :: TODO: Change to a MakeFourCC thing
    class: u8,          // 2 - for 64bit
    data: u8,           // 1 for little endian
    version: u8,        //
    osabi: u8,          // 0 - SYSV
    abi_version: u8,    
    pad: [u8; 6],
    nident: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct ElfHeader
{
    ident: EldHeaderIdent,
    file_type: u16,     // 2 - for Executable
    machine: u16,
    version: u32,

    entry: usize,       //Entrypoint address

    phoff: usize,       //Program header offset
    shoff: usize,       //Section header offset
    
    flags: u32,
    ehsize: u16,        //Elf header size - Assert it is the same size as our structure?

    phentsize: u16,     //Size of program header entry
    phnum: u16,         //Number of program header entries
    shentsize: u16,     //Size of section header entry
    shnum: u16,         //Number of section header entries
    shstrndx: u16,      //Section name string table index
}

#[derive(Debug)]
pub enum ElfError {
    NotEnoughData,
    InvalidMagicNumber,
}

pub struct Elf<'a>
{
    data: &'a[u8],
    header: &'a ElfHeader,
}
impl<'a> Elf<'a>
{
    pub fn from_data(data: &'a [u8]) -> Result<Elf<'a>, ElfError>
    {
        if data.len() < mem::size_of::<ElfHeader>()
        {
            return Err(ElfError::NotEnoughData);
        }

        let header_ptr = &data[0] 
            as *const _ 
            as *const ElfHeader;

        let header = unsafe{header_ptr.as_ref().unwrap()};
        let ident = &header.ident;

        if ident.magic[0] != 127
            || ident.magic[1] != 69
            || ident.magic[2] != 76
            || ident.magic[3] != 70 
        {
                return Err(ElfError::InvalidMagicNumber);
        }

        Ok(Elf {
            data: data,
            header: header,
        })
    }

    pub fn header(&self) -> &ElfHeader
    {
        return self.header
    }

    pub fn program_header(&self) -> Option<&ElfProgramHeaderTable>
    {
        let header_size = self.header.phentsize as usize;
        let header_start = self.header.phoff;
        let header_end = header_start + header_size;

        if header_end >= self.data.len()
        {
            return None;
        }

        let prog_header_ptr = &self.data[header_start] 
            as *const _ 
            as *const ElfProgramHeaderTable;

        let prog_header = unsafe{prog_header_ptr.as_ref().unwrap()};

        Some(prog_header)
    }

    pub fn get_section(&self, idx: u16) -> Option<&'a ElfSectionHeader>
    {
        if idx >= self.header.shnum 
        {
            return None;
        }
        
        let section_size = self.header.shentsize as usize;
        let section_start = self.header.shoff + section_size * idx as usize;
        let section_end = section_start + section_size;

        if section_end >= self.data.len()
        {
            return None;
        }

        let section_ptr = &self.data[section_start] 
            as *const _ 
            as *const ElfSectionHeader;

        let section = unsafe{section_ptr.as_ref().unwrap()};

        Some(section)
    }

    pub fn get_section_data(&self, section: & ElfSectionHeader) -> Option<&[u8]>
    {
        //TODO: Validate the section and bounds etc

        let section_ptr = &self.data[section.file_offset] 
            as *const _ 
            as *const u8;

        let slice = unsafe { slice::from_raw_parts(section_ptr, section.size) }; 

        return Some(slice)
    }

    pub fn sections_iter(&'a self) -> ElfSectionHeaderIter<'a>
    {
        ElfSectionHeaderIter {
            idx: 0,
            elf: self,
        }
    }
}

pub struct ElfSectionHeaderIter<'a>
{
    idx: u16,
    elf: &'a Elf<'a>,
}

impl<'a> Iterator for ElfSectionHeaderIter<'a>
{
    type Item = &'a ElfSectionHeader;
    fn next(&mut self) -> Option<Self::Item> 
    {
        let item = self.elf.get_section(self.idx);
        self.idx += 1;

        item
    }
}

//------------------------------------------------
//TODO: This is something that should live in the kernel

pub enum ExeElfError {
    
}

//TODO: Implement a trait that we need to use when executing an image
pub struct ExecutableElf<'a>
{
    pub elf: &'a Elf<'a>,
}

impl<'a> ExecutableElf<'a>
{
    //TODO: Return a result type
    pub fn from_elf(elf: &'a Elf) -> Result<ExecutableElf<'a>, ExeElfError>
    {
        //TODO: Validate Architecture
        //TODO: Validate Program's virtual address

        Ok(ExecutableElf { elf })
    }
}
