#[allow(identity_op)]
#[cfg(test)]

use std::time::Duration;
use std::thread;
use std::collections::BTreeMap;
use rand;
use serde_json::builder::ObjectBuilder;
use ws::{Sender};

use super::structs::{Town_Material, Infos};
use super::constants::{MATERIALS};
use super::websocket::{spawn_websocket, send_infos};

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
    for i in 0..MATERIALS.len() {
        let material = create_rnd_material(i); 
        infos.town.materials.insert(MATERIALS[i], material);
    }
    infos.town.name = &town_name; 
    send_infos(&infos, &socket);
}

fn create_rnd_material (i: usize) -> Town_Material {
    let amount = (rand::random::<f32>() * 100.0) as u32;
    let buy = (rand::random::<f32>() * 100.0) as u32;
    let sell = (rand::random::<f32>() * 100.0) as u32;
    Town_Material{ amount: amount, buy: buy, sell: sell} 
}
