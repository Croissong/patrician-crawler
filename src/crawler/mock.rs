use std::collections::BTreeMap;
use std::time::Duration;
use std::thread;
use rand;
use ws::{ Sender };

use super::subcrawlers::{ town, ship, player };
use super::crawler::{ Infos };
use super::constants::{ MATERIALS, SERVER_URL };
use super::websocket::{ spawn_websocket, send_infos };

pub fn mock() {
    if let Some(socket) = spawn_websocket(SERVER_URL.to_string()) {
        loop {
            send_random_infos("Luebeck".to_string(), "Titanic".to_string(), &socket);
            send_random_infos("Rostock".to_string(), "Titanic".to_string(), &socket); 
            thread::sleep(Duration::from_millis(1000));
        }
    }
}

fn send_random_infos (town_name: String, ship_name: String, socket: &Sender) {
    let mut ship_mats = BTreeMap::new();
    let mut town_mats = BTreeMap::new();
    for (_, key) in MATERIALS.iter().enumerate() {
        town_mats.insert(key.to_owned(), rnd_town_mat());
        ship_mats.insert(key.to_owned(), rnd_ship_mat()); 
    }
    let infos = Infos {
        date: [1, 2, 1345],
        player: player::Player{ name: "Patrician God".to_string(), gold: rnd_u32() },
        ship: ship::Ship{ materials: ship_mats, name: ship_name },
        town: town::Town{ name: town_name, materials: town_mats,
                          total_weight: rnd_u32(), unknown: rnd_u32() }
    };
    send_infos(&infos, socket);
}

fn rnd_ship_mat() -> ship::ShipMaterial {
    ship::ShipMaterial{ amount: rnd_u32(), average_price: rnd_u32() }
}

fn rnd_town_mat() -> town::TownMaterial {
    town::TownMaterial{ amount: rnd_u32(), buy: rnd_u32(), sell: rnd_u32() } 
}

fn rnd_u32() -> u32 {
    (rand::random::<f32>() * 100.0) as u32
}
