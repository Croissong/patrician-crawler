#[allow(identity_op)]
#[cfg(test)]

use std::time::Duration;
use std::thread;
use rand;
use ws::{Sender};

use super::structs::{TownMaterial, Infos};
use super::constants::{MATERIALS};
use super::websocket::{spawn_websocket, send_infos};
use crawler::process::{Process, get_proc_by_name};

#[test]
fn mock() {
    let socket = spawn_websocket(); 
    let mut infos = Infos::new();
    loop {
        send_random_infos("Luebeck", &mut infos, &socket);
        send_random_infos("Rostock", &mut infos, &socket); 
        thread::sleep(Duration::from_millis(1000));
    }
}

fn send_random_infos (town_name: &'static str, infos: &mut Infos, socket: &Sender) {
    for (_, key) in MATERIALS.iter().enumerate() {
        let material = create_rnd_material(); 
        infos.town.materials.insert(key.clone(), material);
    }
    infos.town.name = town_name; 
    send_infos(infos, socket);
}

fn create_rnd_material () -> TownMaterial {
    let amount = (rand::random::<f32>() * 100.0) as u32;
    let buy = (rand::random::<f32>() * 100.0) as u32;
    let sell = (rand::random::<f32>() * 100.0) as u32;
    TownMaterial{ amount: amount, buy: buy, sell: sell} 
}

#[test]
fn get_town_ref() {
    let process = get_proc_by_name("Patrician3.exe").unwrap();
    let mut TOWN_NAME: [u8; 7] = [0u8; 7];
    process.read_memory(&0x06163C30,
                        &mut TOWN_NAME as *mut _ as *mut _,
                        7);
    println!("{:?}", TOWN_NAME);
    println!("get_town_ref: {}", &TOWN_NAME.iter().map(|b| {
        format!("{}", b.clone() as char)}).collect::<String>()); 
}
