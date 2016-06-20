use std::collections::BTreeMap;
use serde_json;

#[derive(Debug, Serialize)]
pub struct Infos<'a> {
    pub ship: Ship<'a>,
    pub town: Town<'a>,
    pub player: Player,
}
impl<'a> Infos<'a> {
    pub fn new() -> Infos<'a> {
        Infos{ Ship: Ship::new(), town: Town::new(), player: Player::new() }
    } 

    pub fn diff(&self, infos: &Infos) -> Infos {
        let mut diffs = Infos::new();
        diffs.ship = &self.ship.diff(infos.ship);
        diffs.town = &self.town.diff(infos.town);
        diffs.player = &self.player.diff(infos.player);
    }
}

#[derive(Debug, Serialize)]
pub struct Town_Material {
    pub amount: u32,
    pub buy: u32,
    pub sell: u32
}

#[derive(Debug, Serialize)]
struct Town <'a> {
    pub name: &'a str,
    pub materials: BTreeMap<&'a str, Town_Material>,
    pub total_weight: u32,
    pub unknown: u32
}
impl<'a> Town<'a> {
    pub fn new() -> Town<'a> {
        Town{ name: "", materials: BTreeMap::new(), total_weight: 0, unkown: 0 }
    }

    pub fn diff(&self, town: Town) -> Town {
        let 
    }
}

#[derive(Debug, Serialize)]
struct Ship<'a> {
    pub materials: BTreeMap<&'a str, Ship_Material>
}
impl<'a> Ship<'a> {
    pub fn new() -> Ship<'a> {
        Ship{ materials: BTreeMap::new() }
    } 
}

#[derive(Debug, Serialize)]
pub struct Ship_Material {
    pub amount: u32,
    pub average_price: u32
}
impl Ship_Material {
    pub fn is_empty(&self) -> bool {
        &self.amount == 0 && &self.average_price == 0
    }
}

#[derive(Debug, Serialize)]
pub struct Player {
    pub gold: u32
}
impl Player {
    pub fn is_empty(&self) -> bool {
        &self.gold == 0
    }
}
