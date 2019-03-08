use std::io;
use std::str;
use std::fs::File;
use memmap::MmapOptions;

use core::mem;
use core::slice;

#[repr(packed)]
#[derive(Debug)]
pub struct FatFileEntry
{
    name: [u8;11],
    attribute: u8,
    reserved: u8,
    
    creation_time_10ths: u8,
    creation_time_hms: u16,
    creation_date: u16,

    last_access: u16,
    
    high_16: u16,

    last_mod_time: u16,
    last_mod_date: u16,

    low_16: u16,

    file_size: u32,
}

#[repr(packed)]
#[derive(Debug)]
pub struct FatBootRecord
{
    magic: [u8;3],      
    oem: [u8;8],         
    bytes_per_sector: u16,
    sectors_per_cluster: u8,       
    num_reserved_clusters: u16,
    num_fat_tables: u8,   
    num_dir_entries: u16,           // Root entry count     
    total_sectors: u16,
    media_descriptor_type: u8,
    num_sectors_per_fat: u16,       // Table size
    num_sectors_per_track: u16, 
    num_heads: u16,
    num_hidden_sectors: u32,
    large_sector_count: u32,
}

#[derive(Debug)]
pub struct FatComputed
{
    total_root_dir_sectors: u16,

    first_fat_sector: u16,
    first_data_sector: u16,

    total_data_sectors: u16,
    total_clusters: u16,
}

fn main() -> io::Result<()>  {
    
    let file = File::open("test.img")?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    let data = &mmap;

    let header_ptr = &data[0] 
            as *const _ 
            as *const FatBootRecord;

    let header = unsafe{header_ptr.as_ref().unwrap()};
    let header_size = 512;//mem::size_of::<FatBootRecord>();

    // total_root_dir_sectors        
    let root_dir_size = header.num_dir_entries * 32;
    let total_root_dir_sectors = 
        (root_dir_size + header.bytes_per_sector -1) / header.bytes_per_sector;

    let first_fat_sector = header.num_reserved_clusters;
    
    let first_dir_sector = first_fat_sector 
        + (header.num_fat_tables as u16 * header.num_sectors_per_fat);
    
    let first_data_sector = first_dir_sector
        + total_root_dir_sectors;

    //let total_data_sectors: u16,
    //let total_clusters

    let computed = FatComputed{
        total_root_dir_sectors,

        first_fat_sector,
        first_data_sector,

        total_data_sectors: 0,
        total_clusters: 0,        
    };

    println!("{:?}", header);
    println!("{:?}", computed);

    println!("-------------------");

    println!("Address of FAT {:X}", first_fat_sector * header.bytes_per_sector);
    println!("Address of DIR {:X}", first_dir_sector * header.bytes_per_sector);
    println!("Address of DAT {:X}", first_data_sector * header.bytes_per_sector);

    //Lets read the root directory and print out info
    //-----------------------------------------------
    
    for entry_idx in 0..(header.num_dir_entries as usize) {
        let first_dir_byte = (first_dir_sector * header.bytes_per_sector) as usize;
        let entry_byte = first_dir_byte + entry_idx * 32;
        
        let first_byte = data[entry_byte];

        // Unused dir entry
        if first_byte == 0xE5 {
            continue;
        }

        // End of Directory
        if first_byte == 0x00 {
            break;
        }
        
        let file_entry_ptr = &data[entry_byte] 
                as *const _ 
                as *const FatFileEntry;

        let file_entry = unsafe{file_entry_ptr.as_ref().unwrap()};

        // Ignore long file names (for now)
        if (file_entry.attribute & 0xF) == 0xF {
            continue;
        }

        println!("File Entry address: {:X}", entry_byte);
        println!("File Entry: {:?}", file_entry);
   
        // TODO: Make this less picky?
        let file_name = str::from_utf8(&file_entry.name).unwrap();
        println!("FileName: {}", file_name);    
    }

    Ok(())
}
