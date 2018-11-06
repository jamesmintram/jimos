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
    section_type: u32,  //Section type
    flags: u64,         //Section attributes
    addr: usize,        //Virtual address in memory
    file_offset: usize, //Offset in file
    size: usize,        //Size of section
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

pub struct Elf<'a>
{
    data: &'a[u8],
    header: &'a ElfHeader,
}
impl<'a> Elf<'a>
{
    //TODO: Make a Result type and return various error codes
    pub fn from_data(data: &'a [u8]) -> Elf<'a>
    {
        //TODO: Validate len
        //TODO: Validate magic numbers

        let header_ptr = &data[0] 
            as *const _ 
            as *const ElfHeader;

        let header = unsafe{header_ptr.as_ref().unwrap()};

        Elf{
            data: data,
            header: header,
        }
    }

    pub fn header(&self) -> &ElfHeader
    {
        return self.header
    }

    pub fn program_header(&self) -> &ElfProgramHeaderTable
    {
        let prog_header_ptr = &self.data[self.header.phoff] 
            as *const _ 
            as *const ElfProgramHeaderTable;

        let prog_header = unsafe{prog_header_ptr.as_ref().unwrap()};
        prog_header
    }

    pub fn get_section(&self, idx: u16) -> Option<&'a ElfSectionHeader>
    {
        if idx >= self.header.shnum {
            return None;
        }

        let section_ptr_offset = 
            self.header.shoff 
            +  self.header.shentsize as usize * idx as usize;

        let section_ptr = &self.data[section_ptr_offset] 
            as *const _ 
            as *const ElfSectionHeader;

        let section = unsafe{section_ptr.as_ref().unwrap()};

        Some(section)
    }

    pub fn sections_iter(&'a self) -> ElfSectionHeaderIter<'a>
    {
        ElfSectionHeaderIter{
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
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.elf.get_section(self.idx);
        self.idx += 1;

        item
    }
}

//------------------------------------------------

//TODO: Implement a trait that we need to use when executing an image
pub struct ExecutableElf<'a>
{
    pub elf: &'a Elf<'a>,
}

impl<'a> ExecutableElf<'a>
{
    //TODO: Return a result type
    pub fn from_elf(elf: &'a Elf) -> ExecutableElf<'a>
    {
        //TODO: Validate Architecture

        ExecutableElf { elf }
    }
}
