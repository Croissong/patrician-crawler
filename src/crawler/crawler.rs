#[allow(needless_lifetimes)]
use super::addr::{Addresses, get_addresses};
use super::structs::{TownMaterial, Infos, ShipMaterial, Player, Town, Ship};
use super::constants::{MATERIALS, TOWN_NAMES};
use super::process::{Process};
use std::collections::BTreeMap;
use std::str::{from_utf8};

pub struct Crawler {
    process: Process,
    infos: Infos,
    addresses: Addresses
}

impl Crawler {
    pub fn new(process: Process) -> Crawler {
        Crawler{ addresses: get_addresses(&process),
                 process: process,
                 infos: Infos::new() } 
    }
    
    pub fn crawl(&mut self) -> Result<Infos, &str> {
        let town_ref = self.get_town_ref(); 
        match is_valid_town(town_ref) {
            Ok(town_name) => self.get_differences(town_name),
            Err(_) => {
                self.addresses = get_addresses(&self.process);
                Err("No changes...1")
            } 
        }
    }

    fn get_differences(&mut self, town_name: &'static str) -> Result<Infos, &str> {
        let new_infos = self.get_infos(town_name);
        let diff = self.infos.diff(&new_infos); 
        if !diff.is_empty() {
            self.infos = new_infos;
            Ok(diff)
        } else {
            Err("No changes...")
        }
    } 
    
    fn get_infos(&mut self, town_name: &'static str) -> Infos {
        let kontor_block = self.get_kontor_block();
        let (town_materials, ship_materials) = self.get_materials(&kontor_block);
        let town = get_town_info(town_name, &kontor_block, town_materials);
        let ship = get_ship_info(ship_materials);
        let player = self.get_player_info();
        Infos{ ship: ship, town: town, player: player}
    }

    fn get_materials(&self, kontor_block: &[u32; 110 as usize])
                     -> (BTreeMap<&'static str, TownMaterial>, BTreeMap<&'static str, ShipMaterial>) {
        let mut town_materials = BTreeMap::new();
        let mut ship_materials = BTreeMap::new();
        for (i, key) in MATERIALS.iter().enumerate() {
            town_materials.insert(key.clone(), create_town_material(i, kontor_block));
            ship_materials.insert(key.clone(), create_ship_material(i, kontor_block)); 
        }
        (town_materials, ship_materials)
    }

    fn get_player_info(&mut self) -> Player {
        let mut player = Player::new();
        if self.infos.player.is_empty() {
            let player_name = self.get_player_name();
            match from_utf8(player_name) {
                Ok(name) => { println!("Player {} found.", name);
                              player.name = name.clone(); },
                Err(_) => { println!("Please select the Kontor for name identification.");
                            self.addresses.update_player_addr(&self.process); }
            }
        }
        player 
    }
    
    fn get_player_name(&self) -> &'static [u8; 8] {
        const PLAYER_NAME: &'static [u8; 8] = &[0u8; 8];
        self.process.read_memory(&self.addresses.player_name,
                                 &mut PLAYER_NAME as *mut _ as *mut _,
                                 8);
        PLAYER_NAME
    }

    fn get_kontor_block(&self) -> [u32; 110 as usize]{
        let mut block = [0u32; 110 as usize];
        self.process.read_memory (&self.addresses.kontor, &mut block as *mut _ as *mut _, 440);
        block
    }

    fn get_town_ref(&self) -> &'static [u8; 7] {
        const TOWN_NAME: &'static [u8; 7] = &[0u8; 7];
        self.process.read_memory(&self.addresses.town_name,
                                 &mut TOWN_NAME as *mut _ as *mut _,
                                 7);
        TOWN_NAME
    }
}

fn is_valid_town<'a>(town_name: &'a [u8; 7]) -> Result<&'a str, &'a str>{
    println!("{:?}", town_name);
    match from_utf8(town_name) {
        Ok(val) => is_known_town(val),
        Err(_) => Err("invalid town")
    }
}

fn is_known_town(town: &str) -> Result<&str, &str> {
    if TOWN_NAMES.to_vec().contains(&town) {
        Ok(town)
    } else  {
        Err("invalid town")
    }
}

fn create_town_material(i: usize, kontor_block: &[u32; 110]) -> TownMaterial {
    TownMaterial{ amount: kontor_block[i],
                  buy: kontor_block[i+50],
                  sell: kontor_block[i+70] }
}

fn create_ship_material(i: usize, kontor_block: &[u32; 110]) -> ShipMaterial {
    ShipMaterial{ amount: kontor_block[i+25],
                  average_price: kontor_block[i+90] }
}


fn get_town_info(name: &'static str, kontor_block: &[u32; 110 as usize],
                 materials: BTreeMap<&'static str, TownMaterial>) -> Town {
    Town{ name: name,
          materials: materials,
          total_weight: kontor_block[24],
          unknown: kontor_block[50] } 
}

fn get_ship_info(materials: BTreeMap<&'static str, ShipMaterial>) -> Ship {
    Ship{ materials: materials } 
}
