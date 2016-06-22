use super::process::{Process};

const PLAYER_NAME_PTR: Pointer = Pointer{ addr: 0x6EE1B4, offset: 0x10 };
const TOWNNAME_PTR: Pointer = Pointer{ addr: 0x006d9584, offset: 0x724 };
const KONTOR_PTR: Pointer = Pointer{ addr: 0x006D9584, offset: 0x524 };

pub fn get_addresses(process: &Process) -> Addresses {
    let kontor = read_ptr_addr(process, &KONTOR_PTR);
    let town_name = read_ptr_addr(process, &Pointer{ addr: read_ptr_addr(process, &TOWNNAME_PTR),
                                                     offset: 0x0 });
    println!("{:x}", town_name);
    let player_name = read_ptr_addr(process, &PLAYER_NAME_PTR);
    Addresses { kontor: kontor, town_name: town_name, player_name: player_name }    
}

fn read_ptr_addr(process: &Process, ptr: &Pointer) -> u64 {
    let mut ptr_addr: u64 = 0;
    process.read_memory(&ptr.addr, &mut ptr_addr as *mut _ as *mut _, 4);
    ptr_addr + ptr.offset
}

pub struct Pointer {
    pub addr: u64,
    pub offset: u64
}

pub struct Addresses {
    pub kontor: u64,
    pub town_name: u64,
    pub player_name: u64 
}

impl Addresses {
    pub fn update_player_addr(&mut self, process: &Process) {
        self.player_name = read_ptr_addr(process, &PLAYER_NAME_PTR); 
    }
}
