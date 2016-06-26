#[allow(identity_op)]

use std::time::Duration;
use std::thread;
use rand;
use ws::{Sender};

use super::structs::{TownMaterial, Infos, Ship, Town, ShipMaterial};
use super::constants::{MATERIALS, SERVER_URL};
use super::websocket::{spawn_websocket, send_infos};

pub fn mock() {
    if let Some(socket) = spawn_websocket(SERVER_URL.to_string()) {
        let mut infos = Infos::new();
        infos.player.name = "Patrician God".to_string();
        loop {
            send_random_infos("Luebeck".to_string(), "Titanic".to_string(), &mut infos, &socket);
            send_random_infos("Rostock".to_string(), "Titanic".to_string(), &mut infos, &socket); 
            thread::sleep(Duration::from_millis(1000));
        }
    }
}

fn send_random_infos (town_name: String, ship_name: String, infos: &mut Infos, socket: &Sender) {
    for (_, key) in MATERIALS.iter().enumerate() {
        infos.town.materials.insert(key.to_owned(), rnd_town_mat());
        infos.ship.materials.insert(key.to_owned(), rnd_ship_mat()); 
    }
    infos.ship = Ship{ materials: infos.ship.materials.clone(), name: ship_name };
    infos.town = Town{ name: town_name, materials: infos.town.materials.clone(),
                       total_weight: rnd_u32(), unknown: rnd_u32() };
    infos.player.gold = rnd_u32();
    send_infos(infos, socket);
}

fn rnd_ship_mat() -> ShipMaterial {
    ShipMaterial{ amount: rnd_u32(), average_price: rnd_u32() }
}

fn rnd_town_mat() -> TownMaterial {
    TownMaterial{ amount: rnd_u32(), buy: rnd_u32(), sell: rnd_u32() } 
}

fn rnd_u32() -> u32 {
    (rand::random::<f32>() * 100.0) as u32
}
