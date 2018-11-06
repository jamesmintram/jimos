/*
    Documenation used: https://www.uclibc.org/docs/elf-64-gen.pdf
*/

use std::io;
use std::fs::File;
use memmap::MmapOptions;

mod elf;


fn main() -> io::Result<()> 
{
    //Read elf header
    let file = File::open("../../../csrc/build/prog1.elf")?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    let elf = elf::Elf::from_data(&mmap);
    let exe_elf = elf::ExecutableElf::from_elf(&elf);
    
    println!("Header: {:#?}", exe_elf.elf.header());
    println!("Prog Header: {:#?}", exe_elf.elf.program_header());

    println!("\n\nSECTIONS:\n\n",);

    for section in elf.sections_iter()
    {
        println!("Section: {:#?}", section);
    }

    Ok(())
}
