use std::collections::BTreeMap;
use super::super::utils::{ diff_mats };

pub fn create_ship_material(i: usize, kontor_block: &[u32; 110]) -> ShipMaterial {
    ShipMaterial{ amount: kontor_block[i+25],
                  average_price: kontor_block[i+90] }
}

pub fn get_ship(materials: BTreeMap<&'static str, ShipMaterial>) -> Ship {
    Ship{ materials: materials, name: "".to_string() } 
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Ship {
    pub materials: BTreeMap<&'static str, ShipMaterial>,
    pub name: String
}
impl Ship {
    pub fn new() -> Ship {
        Ship{ materials: BTreeMap::new() , name: "".to_string() }
    }
    
    pub fn diff(&self, ship: &Ship) -> Ship {
        let mut diff= Ship::new();
        if self.name != ship.name {
            diff.name = ship.name.clone(); 
            diff.materials = ship.materials.clone(); 
        } else {
            diff.name = self.name.clone();
            diff.materials = diff_mats(&self.materials, &ship.materials);
        }
        diff
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

