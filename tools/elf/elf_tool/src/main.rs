/*
    Documenation used: https://www.uclibc.org/docs/elf-64-gen.pdf
*/

use core::mem;
use core::slice;

use std::fs::File;
use std::io;
use std::io::SeekFrom;
use std::io::prelude::*;

const EI_NIDENT: usize = 16;

#[repr(C)]
#[derive(Debug)]
struct ElfProgramHeaderTable
{
    seg_type: u32, // 1 - Loadable segment
    flags: u32,    // 0x1 - Execute, 0x2 - Write, 0x4 - Read
    offset: usize, // Offset in file
    vaddr: usize,  // Virtual address in memory
    paddr: usize,
    file_size: u64,//Size of segment in file
    mem_size: u64, //Size of segment in memory
    align: u64,    //Alignment of segment
}

#[repr(C)]
#[derive(Debug)]
struct ElfSectionHeader 
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
struct EldHeaderIdent
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
struct ElfHeader
{
    ident: EldHeaderIdent,
    file_type: u16,     // 2 - for Executable
    machine: u16,
    version: u32,

    entry: usize,       //Entrypoint address

    phoff: u64,       //Program header offset
    shoff: u64,       //Section header offset
    
    flags: u32,
    ehsize: u16,        //Elf header size - Assert it is the same size as our structure?

    phentsize: u16,     //Size of program header entry
    phnum: u16,         //Number of program header entries
    shentsize: u16,     //Size of section header entry
    shnum: u16,         //Number of section header entries
    shstrndx: u16,      //Section name string table index
}

fn main() -> io::Result<()> {
    let mut header: ElfHeader = unsafe { mem::zeroed() };
    let header_slice = unsafe {slice::from_raw_parts_mut(
        &mut header as *mut _ as *mut u8,
        mem::size_of::<ElfHeader>()
    )};

    let mut prog_header: ElfProgramHeaderTable = unsafe {mem::zeroed() };
    let prog_slice = unsafe {slice::from_raw_parts_mut(
        &mut prog_header as *mut _ as *mut u8,
        mem::size_of::<ElfProgramHeaderTable>()
    )};



    //Read elf header
    let mut f = File::open("../../../csrc/build/prog1.elf").expect("File not found");
    f.read_exact(header_slice).expect("Invalid read");
    
    println!("Header: {:#?}", header);

    // // Read string table into slice (or slice::from_raw_parts)
    // let mut section_slice = Vec::with_capacity(header.
        
    //     mem::size_of::<ElfSectionHeader>()
    // );

    
    // Read program header
    f.seek(SeekFrom::Start(header.phoff))?;
    f.read_exact(prog_slice).expect("Invalid read");

    println!("Prog Header: {:#?}", prog_header);


    // Read section headers
    f.seek(SeekFrom::Start(header.shoff))?;
    
    let mut section_headers = Vec::new();

    for idx in 0..header.shnum {
        let mut section_header: ElfSectionHeader = unsafe {mem::zeroed()};
        let section_slice = unsafe {slice::from_raw_parts_mut(
            &mut section_header as *mut _ as *mut u8,
            mem::size_of::<ElfSectionHeader>()
        )};

        f.read_exact(section_slice).expect("Invalid read");
        println!("Section Header: {:#?}", section_header);

        section_headers.push(section_header);
    }

    println!("\n\n");
    println!("Section Header for string table: \n{:#?}", section_headers[header.shstrndx as usize]);

    Ok(())
}
