#[allow(needless_lifetimes)]
use super::addr::{Addresses, get_addresses};
use super::structs::{TownMaterial, Infos, ShipMaterial, Player, Town, Ship};
use super::constants::{MATERIALS, TOWN_NAMES};
use super::process::{Process};
use std::collections::BTreeMap;

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
        let town_name = self.get_town_name(); 
        if is_known_town(&town_name) {
            self.get_differences(town_name)
        } else {
            self.addresses = get_addresses(&self.process);
            Err("Invalid town") 
        }
    }

    fn get_differences(&mut self, town_name: String) -> Result<Infos, &str> {
        let new_infos = self.get_infos(town_name);
        let diff = self.infos.diff(&new_infos); 
        if !diff.is_empty() {
            self.infos = new_infos;
            Ok(diff)
        } else {
            Err("No changes")
        }
    } 
    
    fn get_infos(&mut self, town_name: String) -> Infos {
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
            let name = self.get_player_name();
            if is_valid_player_name(&name) {
                println!("Player {} found.", name);
                player.name = name;
            } else {
                println!("Please select the Kontor for name identification.");
                self.addresses.update_player_addr(&self.process); 
            }
        } 
        player 
    }
    
    fn get_player_name(&self) -> String {
        let mut player_name_arr = [0u8; 8];
        self.process.read_memory(&self.addresses.player_name,
                                 &mut player_name_arr as *mut _ as *mut _,
                                 8);
        player_name_arr.iter()
            .map(|b| { format!("{}", b.clone() as char) })
            .collect::<String>()
    }

    fn get_kontor_block(&self) -> [u32; 110 as usize]{
        let mut block = [0u32; 110 as usize];
        self.process.read_memory (&self.addresses.kontor, &mut block as *mut _ as *mut _, 440);
        block
    }

    fn get_town_name(&self) -> String {
        let mut town_name_arr = [0u8; 7];
        self.process.read_memory(&self.addresses.town_name,
                                 &mut town_name_arr as *mut _ as *mut _,
                                 7); 
        town_name_arr.iter()
            .map(|b| { format!("{}", b.clone() as char) })
            .collect::<String>()
    }
}
fn is_known_town(town: &str) -> bool {
    TOWN_NAMES.contains(&town)
}
fn is_valid_player_name(name: &str) -> bool {
    name.len() > 2
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


fn get_town_info(name: String, kontor_block: &[u32; 110 as usize],
                 materials: BTreeMap<&'static str, TownMaterial>) -> Town {
    Town{ name: name,
          materials: materials,
          total_weight: kontor_block[24],
          unknown: kontor_block[50] } 
}

fn get_ship_info(materials: BTreeMap<&'static str, ShipMaterial>) -> Ship {
    Ship{ materials: materials } 
}
