use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
pub struct Infos {
    pub ship: Ship,
    pub town: Town,
    pub player: Player,
}
impl Infos {
    pub fn new() -> Infos {
        Infos{ ship: Ship::new(), town: Town::new(), player: Player::new() }
    } 

    pub fn diff(&self, infos: &Infos) -> Infos {
        let mut diffs = Infos::new();
        diffs.ship = self.ship.diff(&infos.ship);
        diffs.town = self.town.diff(&infos.town);
        diffs.player = self.player.diff(&infos.player);
        diffs
    }

    pub fn is_empty(&self) -> bool {
        self.ship.is_empty() && self.town.is_empty() && self.player.is_empty()
    }
}

#[derive(Debug, Serialize, PartialEq, Clone, Copy)]
pub struct TownMaterial {
    pub amount: u32,
    pub buy: u32,
    pub sell: u32
}

#[derive(Debug, Serialize)]
pub struct Town  {
    pub name: &'static str,
    pub materials: BTreeMap<&'static str, TownMaterial>,
    pub total_weight: u32,
    pub unknown: u32
}
impl Town {
    pub fn new() -> Town {
        Town{ name: "", materials: BTreeMap::new(), total_weight: 0, unknown: 0 }
    }

    pub fn diff(&self, town: &Town) -> Town {
        let mut diff: Town = Town::new();
        if &self.name != &town.name {
            diff.name = town.name.clone();
            // if new town don't diff mats
            diff.materials = town.materials.clone();
            diff.total_weight = town.total_weight.clone();
            diff.unknown = town.unknown.clone();
        }
        diff.materials = diff_mats(&self.materials, &town.materials);
        if &self.total_weight != &town.total_weight {
            diff.total_weight = town.total_weight.clone();
        }
        if &self.unknown != &town.unknown {
            diff.unknown = town.unknown.clone();
        }
        diff
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_empty() && self.materials.is_empty()
            && &self.total_weight == &0 && &self.unknown == &0 
    }
}

#[derive(Debug, Serialize)]
pub struct Ship {
    pub materials: BTreeMap<&'static str, ShipMaterial>
}
impl Ship {
    pub fn new() -> Ship {
        Ship{ materials: BTreeMap::new() }
    }
    
    pub fn diff(&self, ship: &Ship) -> Ship {
        Ship{ materials: diff_mats(&self.materials, &ship.materials) }
    }

    pub fn is_empty(&self) -> bool {
        self.materials.is_empty()
    }
}

#[derive(Debug, Serialize, PartialEq, Clone, Copy)]
pub struct ShipMaterial {
    pub amount: u32,
    pub average_price: u32
}

#[derive(Debug, Serialize)]
pub struct Player {
    pub gold: u32,
    pub name: &'static str
}
impl Player {

    pub fn new() -> Player {
        Player{name: "", gold: 0}
    }
    
    pub fn is_empty(&self) -> bool {
        &self.gold == &0
    }

    pub fn diff(&self, player: &Player) -> Player {
        let mut diff = Player::new();
        if &self.gold != &player.gold {
            diff.gold = player.gold;
        }
        diff
    } 
}

fn diff_mats<'a, T: PartialEq + Clone>(mats: &BTreeMap<&'static str, T>,
                                       new_mats: &BTreeMap<&'static str, T>)
                                       -> BTreeMap<&'static str, T> {
    let mut diff: BTreeMap<&str, T> = new_mats.clone(); 
    for (key, val ) in new_mats.iter() {
        if &mats.get(key).unwrap() == &val {
            diff.remove(key);
        }
    }
    diff
}
