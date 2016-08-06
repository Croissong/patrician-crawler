use super::super::process::{Process, Pointer};

pub struct PlayerCrawler {
    process: Process, 
    first_name_addr: u64,
    last_name_addr: u64,
    gold_addr: u64
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

    pub fn get_info(&self, curr_player: &Player) -> Player {
        Player{ name: self.get_name(curr_player),
                gold: self.get_gold() } 
    }

    fn get_name(&self, curr_player: &Player) -> String {
        if curr_player.name.is_empty() {
            self.read_name() 
        } else {
            curr_player.name.clone()
        }
    }
    
    fn get_gold(&self) -> u32 {
        let mut gold = 0;
        self.process.read_memory(&self.gold_addr,
                                 &mut gold as *mut _ as *mut _,
                                 4);
        gold
    }
    
    fn read_name(&self) -> String {
        let mut first_name = [0u8; 14];
        let mut last_name = [0u8; 14];
        self.process.read_memory(&self.first_name_addr,
                                 &mut first_name as *mut _ as *mut _,
                                 14);
        self.process.read_memory(&self.last_name_addr,
                                 &mut last_name as *mut _ as *mut _,
                                 14);
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
    pub name: String
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
