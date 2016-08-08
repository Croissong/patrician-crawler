use super::super::constants::{ MATERIALS };
use super::super::process::{ Process, Pointer};
use super::town::{ Town, TownMaterial, get_town, create_town_material };
use super::ship::{ Ship, ShipMaterial, get_ship, create_ship_material };

use std::collections::BTreeMap;

pub struct KontorCrawler {
    process: Process,
    kontor_addr: u32
}

impl KontorCrawler {
    pub fn new(process: Process) -> KontorCrawler {
        let kontor_ptr = Pointer{ addr: 0x006D9584, offsets: vec![0x524] };
        KontorCrawler { process: process, kontor_addr: process.read_ptr(&kontor_ptr) }
    }

    pub fn get_info(&self, town_name: String) -> (Town, Ship) {
        let kontor_block = self.get_kontor_block();
        let (town_materials, ship_materials) = self.get_materials(&kontor_block);
        let town = get_town(town_name, &kontor_block, town_materials);
        let ship = get_ship(ship_materials);
        (town, ship)
    }

    fn get_kontor_block(&self) -> [u32; 110 as usize]{
        self.process.read_memory (&self.kontor_addr, [0u32; 110 as usize]) 
    }

    fn get_materials(&self, kontor_block: &[u32; 110 as usize])
                     -> ( BTreeMap<&'static str, TownMaterial>,
                          BTreeMap<&'static str, ShipMaterial> )
    {
        let mut town_materials = BTreeMap::new();
        let mut ship_materials = BTreeMap::new();
        for (i, key) in MATERIALS.iter().enumerate() {
            town_materials.insert(key.to_owned(), create_town_material(i, kontor_block));
            ship_materials.insert(key.to_owned(), create_ship_material(i, kontor_block)); 
        }
        (town_materials, ship_materials)
    }
}
