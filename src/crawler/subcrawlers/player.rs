use super::super::process::{ Process, Pointer };
use super::super::crawler::{ Infos };

pub struct PlayerCrawler {
    process: Process, 
    first_name_addr: u32,
    last_name_addr: u32,
    gold_addr: u32
}

impl PlayerCrawler {
    pub fn new(process: Process) -> PlayerCrawler {
        let first_name_ptr = Pointer{ addr: 0x6d1c7c, offsets: vec![0x1b48, 0x0] };
        let last_name_ptr = Pointer{ addr: 0x6d1c7c, offsets: vec![0x1b48, -0x50] };
        let gold_ptr = Pointer{ addr: 0x6cbb40, offsets: vec![0x7c0] }; 
        PlayerCrawler { process: process, gold_addr: process.read_ptr(&gold_ptr),
                        first_name_addr: process.read_ptr(&first_name_ptr),
                        last_name_addr: process.read_ptr(&last_name_ptr) }
    }

    pub fn get_info(&self, old: Option<&Infos>) -> Player {
        Player{ name: self.get_name(old),
                gold: self.get_gold() } 
    }

    fn get_name(&self, old: Option<&Infos>) -> Option<String> {
        match old {
            Some(_name) => None,
            None => Some(self.read_name())
        } 
    }
    
    fn get_gold(&self) -> u32 {
        self.process.read_memory(&self.gold_addr, 0u32)
    }
    
    fn read_name(&self) -> String {
        let first_name = self.process.read_memory(&self.first_name_addr, [0u8; 14]);
        let last_name = self.process.read_memory(&self.last_name_addr, [0u8; 14]);
        let first_name = first_name.iter()
            .map(|b| { format!("{}", b.clone() as char) })
            .collect::<String>();
        let last_name = last_name.iter()
            .map(|b| { format!("{}", b.clone() as char) })
            .collect::<String>();
        format!("{} {}", first_name, last_name)
    }
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Player {
    pub gold: u32,
    pub name: Option<String>
}
impl Player {
    
    pub fn is_empty(&self) -> bool {
        &self.gold == &0
    } 

    pub fn diff(&self, player: &Player) -> Player {
        let mut diff = Player{ name: player.name.clone(), gold: 0 };
        if &self.gold != &player.gold {
            diff.gold = player.gold;
        }
        diff
    } 
}
