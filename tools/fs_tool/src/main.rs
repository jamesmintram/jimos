use std::io;
use std::fs::File;
use memmap::MmapOptions;

use core::mem;
use core::slice;

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

    //println!("Size of header: {}", );

    Ok(())
}
