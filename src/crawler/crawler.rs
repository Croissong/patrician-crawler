use ws::{Sender};
use super::addr{Addresses, get_addresses};
use super::structs::{Town_Material, Infos, Ship_Material};
use super::process::{Process};

pub struct Crawler {
    socket: Sender,
    process: Process,
    mut infos: Infos,
    mut addresses: Addresses
}

impl Crawler {
    pub fn new(socket: Sender, addresses: Addresses, process: Process) -> Crawler {
        Crawler{ socket: socket,
                 addresses: get_addresses(&process),
                 process: process,
                 infos: Infos::new() } 
    }
    
    pub fn crawl(&self) -> Result<&Infos, &str> {
        let town_ref = &self.get_town_ref(); 
        match is_valid_town(&town_ref) {
            Ok(town_name) => &self.crawl_infos(&town_name),
            Err(_) => {
                &self.addresses = get_addresses(&process);
                Err("No changes...")
            } 
        }
    }
    
    fn crawl_infos(&self, town_name: &str) -> Result<&Infos, &str> {
        let mut changes = Infos::new();
        &self.update_town(&kontor_block, &changes);
        if &self.infos.player.is_empty() {
            &self.update_player(); 
        } 
        &self.update_infos(&town_name, &kontor_block);
        match &self.infos.has_changes() {
            true => Ok(&self.infos),
            false => Err("No changes...")
        }
    }

    fn update_town(&self, town_name: &str, changes: &Infos) {
        let kontor_block = &self.get_kontor_block();
        &self.update_materials();
        &self.town.
            let town = ObjectBuilder::new()
            .insert("amount", materials[24])
            .insert("name", town_name)
            .insert("y".to_string(), materials[50])
            .unwrap(); 
        infos.town = town; 
    }

    fn update_player(&self){
        let player_name = get_player_name();
        if let Ok(player_name) = is_valid_player_name(&player_name){
            infos.player = player_name.to_string();
            if infos.player != "???".to_string() {
                println!("Player {} found.", infos.player);
            } else {
                println!("Please select the Kontor for name identification.");
                addresses.2 = get_player_name_addr(&process);
            }
        }
    } 

    fn update_materials(kontor_block: &[u32; 110 as usize], changes: &mut Infos) {
        let prev_materials = infos.materials; 
        let changed_materials = BTreeMap::new();
        for i in 0..MATERIALS.len() {
            let material = create_material(i, &materials);
            {
                let prev_material = prev_materials.get(MATERIALS[i]).unwrap();
                if prev_material != &material && !material.is_empty(){
                    changed_materials.insert(MATERIALS[i], material);
                }
            }
        } 
        infos.materials = changed_materials;
        (materials)
    }

    fn create_town_material(i: usize, kontor_block: [u32; 110]) -> Town_Material {
        Town_Material{ amount: kontor_block[i],
                       buy: kontor_block[i+50],
                       sell: kontor_block[i+70] }
    }

    fn create_ship_material(i: usize, kontor_block: [u32; 110]) -> Ship_Material {
        Ship_Material{ amount: kontor_block[i+25],
                       average: kontor_block[i+90] }
    }

    fn get_kontor_block(&self) -> [u32; 110 as usize]{
        let mut block = [0u32; 110 as usize];
        &self.process.read_memory (&self.addresses.kontor, &mut block as *mut _ as *mut _, 440);
        block
    }

    fn get_town_ref(&self) -> [u8; 7] {
        let mut town_name = [0u8; 7];
        &self.process.read_memory(&self.addresses.town_name,
                                  &mut town_name as *mut _ as *mut _,
                                  7);
        town_name
    }

    fn get_player_name(&self) -> [u8; 8] {
        let mut player_name = [0u8; 8];
        &self.process.read_memory(&self.addresses.player_name,
                                  &mut player_name as *mut _ as *mut _,
                                  8);
        player_name
    }

    fn is_valid_player_name<'a>(player_name: &'a [u8; 8]) -> Res<&'a str, &str> {
        match std::str::from_utf8(player_name) {
            Ok(val) => Ok(val),
            Err(_) => Err("invalid town")
        }
    }

    fn is_valid_town<'a>(town_name: &'a [u8; 7]) -> Res<&'a str, &str>{
        match std::str::from_utf8(town_name) {
            Ok(val) => is_known_town(val),
            Err(_) => Err("invalid town")
        }
    }
    
    fn is_known_town(town: &str) -> Res<&str, &str> {
        match TOWN_NAMES.to_vec().contains(&town) {
            true => Ok(town),
            false => Err("invalid town")
        }
    }
}
