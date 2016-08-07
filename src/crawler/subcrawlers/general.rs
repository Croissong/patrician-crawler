use super::super::constants::{TOWN_NAMES};
use super::super::process::{ Process, Pointer };

pub struct GeneralCrawler {
    process: Process,
    town_name_addr: u64,
    date_addr: u64
}

impl GeneralCrawler {
    pub fn new(process: Process) -> GeneralCrawler {
        let townname_ptr = Pointer{ addr: 0x006d9584, offsets: vec![0x724, 0x0] };
        let date_ptr = Pointer{ addr: 0x006D8D60, offsets: vec![0x0] };
        GeneralCrawler{ process: process, town_name_addr: process.read_ptr(&townname_ptr),
                        date_addr: process.read_ptr(&date_ptr) }
    }
    
    pub fn get_town_name(&self) -> Option<String> {
        let mut town_name_arr = [0u8; 11];
        self.process.read_memory(&self.town_name_addr,
                                 &mut town_name_arr as *mut _ as *mut _,
                                 11); 
        let mut name = town_name_arr.iter()
            .map(|b| { format!("{}", b.clone() as char) })
            .collect::<String>(); 
        name = name.split(' ').next().unwrap().to_string();
        if TOWN_NAMES.contains(&name.as_str()) {
            Some(name)
        } else {
            println!("unkown town: {} at addr: {:x}", name, &self.town_name_addr);
            None
        }
    }

    pub fn get_date(&self) -> [u32; 3] {
        let mut date_arr = [0u32; 3];
        self.process.read_memory(&self.date_addr,
                                 &mut date_arr as *mut _ as *mut _,
                                 12);
        println!("{}, {}, {}", date_arr[0], date_arr[1], date_arr[2]);
        date_arr
    }
}

