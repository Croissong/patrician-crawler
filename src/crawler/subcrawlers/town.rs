use super::super::utils::{ diff_mats };
use std::collections::BTreeMap;

pub fn get_town(name: String, kontor_block: &[u32; 110 as usize],
                materials: BTreeMap<&'static str, TownMaterial>) -> Town {
    Town{ name: name,
          materials: materials,
          total_weight: kontor_block[24],
          unknown: kontor_block[50] } 
}

pub fn create_town_material(i: usize, kontor_block: &[u32; 110]) -> TownMaterial {
    TownMaterial{ amount: kontor_block[i],
                  buy: kontor_block[i+50],
                  sell: kontor_block[i+70] }
}

#[derive(Debug, Serialize, PartialEq, Clone, Copy)]
pub struct TownMaterial {
    pub amount: u32,
    pub buy: u32,
    pub sell: u32
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Town  {
    pub name: String,
    pub materials: BTreeMap<&'static str, TownMaterial>,
    pub total_weight: u32,
    pub unknown: u32
}
impl Town {
    pub fn new() -> Town {
        Town{ name: "".to_string(), materials: BTreeMap::new(), total_weight: 0, unknown: 0 }
    }

    pub fn diff(&self, town: &Town) -> Town {
        let mut diff = Town::new();
        if &self.name != &town.name {
            diff.name = town.name.clone();
            // if new town don't diff mats
            diff.materials = town.materials.clone();
            diff.total_weight = town.total_weight;
            diff.unknown = town.unknown;
        } else {
            diff.name = self.name.clone();
            diff.materials = diff_mats(&self.materials, &town.materials);
        }
        if &self.total_weight != &town.total_weight {
            diff.total_weight = town.total_weight;
        }
        if &self.unknown != &town.unknown {
            diff.unknown = town.unknown;
        }
        diff
    }

    pub fn is_empty(&self) -> bool {
        self.materials.is_empty()
            && &self.total_weight == &0 && &self.unknown == &0 
    }
}
