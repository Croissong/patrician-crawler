use super::super::constants::{TOWN_NAMES};
use super::super::process::{ Process, Pointer };
use super::super::crawler::{ Infos };
use std::str::{ from_utf8 };

pub struct GeneralCrawler {
    process: Process,
    town_name_addr: u32,
    date_addr: u32
}

impl GeneralCrawler {
    pub fn new(process: Process) -> GeneralCrawler {
        let townname_ptr = Pointer{ addr: 0x006d9584, offsets: vec![0x724, 0x0] }; 
        GeneralCrawler{ process: process, town_name_addr: process.read_ptr(&townname_ptr),
                        date_addr: 0x006D8D60 }
    }
    
    pub fn get_town_name(&self) -> Option<String> {
        let town_name_arr = self.process.read_memory(&self.town_name_addr,
                                                     [0u8; 11]); 
        let mut name = from_utf8(&town_name_arr).unwrap();
        name = name.split(' ').next().unwrap();
        if TOWN_NAMES.contains(&name) {
            Some(name.to_string())
        } else {
            println!("unkown town: {} at addr: {:x}", name, &self.town_name_addr);
            None
        }
    }

    pub fn get_date(&self, old: Option<&Infos>) -> [u32; 3] {
        let mut date = None;
        while date == None {
            let mut date_arr = self.process.read_memory(&self.date_addr, [0u32; 3]); 
            if date_arr[0] <= 31 && date_arr[0] > 0 {
                date_arr[1] = (date_arr[1] - 7081168) / 4; 
                date = Some(date_arr);
            } else if let Some(infos) = old {
                return infos.date;
            }
        }
        date.unwrap()
    }
}
