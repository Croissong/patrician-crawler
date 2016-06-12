extern crate kernel32;
extern crate winapi;
extern crate wio;
extern crate time;
extern crate serde;
extern crate serde_json;
extern crate ws;
extern crate env_logger;
#[macro_use] extern crate log;

use std::mem;
use std::ffi::OsString;
use wio::wide::FromWide;
use std::ptr;
use std::os::raw::c_void;
use std::collections::BTreeMap;
use ws::{connect, Handler, Sender, Handshake, Result, Message};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use serde_json::builder::ObjectBuilder;
use serde_json::builder::ArrayBuilder;
use std::result::Result as Res;


struct Client {
    out: Sender,
    tx: std::sync::mpsc::Sender<Sender>
        
}

const MATERIALS: [(&'static str, usize); 20] = [("grain", 4), ("meat", 9), ("fish", 3), ("beer", 0), ("salt", 13), ("honey", 6), ("spices", 15), ("wine", 18), ("cloth", 2), ("skins", 14), ("whale oil", 17), ("timber", 16), ("iron goods", 7), ("leather", 8), ("wool", 19), ("pitch", 11), ("pig iron", 10), ("hemp", 5), ("pottery", 12), ("bricks", 1)];

const TOWN_NAMES: [&'static str; 3] = ["Luebeck", "Rostock", "Hamburg"];

const NAME_STATIC : (u64, u64) = (0x6EE1B4, 0x10);
const TOWNNAME_STATIC: (u64, u64) = (0x006d9584, 0x724);
const MATERIALS_STATIC: (u64, u64) = (0x006D9584, 0x524);

fn join_channel(out: &ws::Sender){
    let str = "{\"topic\":\"rust_client:web_client\",\"ref\":\"1\",\"payload\":{},\"event\":\"phx_join\"}"; 
    out.send(str).unwrap();
}

fn get_town_address(pid: u32) -> u64 {
    let mut pointer: u64 = 0;
    let mut town_address: u64 = 0;
    get_proc(pid).read_memory(&TOWNNAME_STATIC.0,
                              &mut pointer as *mut _ as *mut _,
                              4);
    pointer += TOWNNAME_STATIC.1;
    get_proc(pid).read_memory(&pointer,
                              &mut town_address as *mut _ as *mut _,
                              4);
    town_address
}

fn get_name_address(pid: u32) -> u64 {
    let mut name_address: u64 = 0;
    get_proc(pid).read_memory(&NAME_STATIC.0,
                              &mut name_address as *mut _ as *mut _,
                              4);
    name_address + NAME_STATIC.1 
}

fn get_materials_addr(pid: u32) -> u64 {
    let mut materials_addr: u64 = 0;
    get_proc(pid).read_memory(&MATERIALS_STATIC.0,
                              &mut materials_addr as *mut _ as *mut _,
                              4); 
    materials_addr + MATERIALS_STATIC.1
}

fn spawn_websocket() -> ws::Sender{
    let (tx, rx) = channel(); 
    thread::spawn(move|| {
        connect("ws://localhost:4000/socket/websocket?vsn=1.0.0", |out| {
            Client { out: out, tx: tx.clone() }
        }).unwrap();             // send the Sender to thread 1

    });
    let socket: ws::Sender = rx.recv().unwrap();
    join_channel(&socket);
    socket
}

fn main() {
    env_logger::init().unwrap();
    match get_proc_id_by_name("Patrician3.exe") {
        0 => println!("No Patrician process found."),
        pid => start_crawler(pid)
    }
}

fn start_crawler(pid: u32) {
    println!("Found Patrician process with pid {}.", pid);
    let socket = spawn_websocket(); 
    let mut addresses = get_addresses(pid);
    let mut infos = Infos::new();
    
    loop {
        let town_ref = get_town_name(pid, &addresses.1); 
        match is_valid_town(&town_ref) {
            Ok(town_name) => crawl_infos(pid, &mut addresses, &mut infos, &town_name, &socket),
            Err(_) => addresses = get_addresses(pid)
        }
        thread::sleep(Duration::from_millis(1000));
    }
}

fn get_addresses(pid: u32) -> (u64, u64, u64){
    (get_materials_addr(pid), get_town_address(pid), get_name_address(pid))
}

fn crawl_infos(pid: u32, mut addresses: &mut (u64, u64, u64), mut infos: &mut Infos, town_name: &str, socket: &ws::Sender) {
    let (materials, dirty_flag) = update_materials(pid, &addresses.0, &mut infos);
    if infos.player == "???".to_string() {
        update_player(pid, &mut addresses, &mut infos); 
    }
    if dirty_flag {
        update_infos(&town_name, &materials, &mut infos);
        send_infos(&infos, &socket);
    } else {
        println!("No changes...")
    }
}

fn send_infos(infos: &Infos, socket: &ws::Sender) {
    let json = &infos.serialize();
    let string = format!("{{\"topic\":\"rust_client:web_client\",\"ref\":null,\"payload\":{{\"body\":{}}},\"event\":\"rust_client:web_client\"}}", json);
    println!("sent update at {}", time::now().ctime());
    socket.send(string).unwrap();
}

fn update_infos(town_name: &str, materials: &[u32; 110 as usize],
                mut infos: &mut Infos) {
    let town = ObjectBuilder::new()
        .insert("amount", materials[24])
        .insert("name", town_name)
        .insert("y".to_string(), materials[50])
        .unwrap(); 
    infos.town = town; 
}

fn update_player(pid: u32, addresses: &mut (u64, u64, u64), infos: &mut Infos){
    let player_name = get_player_name(pid, &addresses.2);
    if let Ok(player_name) = is_valid_player_name(&player_name){
        infos.player = player_name.to_string();
        if infos.player != "???".to_string() {
            println!("Player {} found.", infos.player);
        } else {
            println!("Please select the Kontor for name identification.");
            addresses.2 = get_name_address(pid);
        }
    }
} 


fn update_materials(pid: u32, materials_address: &u64,
                    infos: &mut Infos) -> ([u32; 110 as usize], bool) {
    let mut dirty_flag = false;
    let materials = get_materials(pid, &materials_address);
    for i in 0..MATERIALS.len() {
        let material = create_material(i, &materials); 
        if infos.materials[MATERIALS[i].1] != material && !material.is_empty(){
            dirty_flag = true;    
        }
        infos.materials[MATERIALS[i].1] = material;
    }
    (materials, dirty_flag)
}

fn create_material(i: usize, materials: &[u32; 110]) -> Material{
    let mut values = BTreeMap::new();
    values.insert("amount".to_string(), materials[i]);
    values.insert("buy".to_string(), materials[i+50]);
    values.insert("sell".to_string(), materials[i+70]);
    values.insert("office".to_string(), materials[i+25]);
    values.insert("average".to_string(), materials[i+90]);
    Material::new(MATERIALS[i].0.to_string(), values)
}

fn get_materials(pid: u32, materials_address: &u64) -> [u32; 110 as usize]{
    let mut block = [0u32; 110 as usize];
    get_proc(pid).read_memory (&materials_address, &mut block as *mut _ as *mut _, 440);
    block
}

fn get_town_name(pid: u32, town_address: &u64) -> [u8; 7] {
    let mut town_name = [0u8; 7];
    get_proc(pid).read_memory(&town_address,
                              &mut town_name as *mut _ as *mut _,
                              7);
    town_name
}

fn get_player_name(pid: u32, player_address: &u64) -> [u8; 8] {
    let mut player_name = [0u8; 8];
    get_proc(pid).read_memory(&player_address,
                              &mut player_name as *mut _ as *mut _,
                              8);
    player_name
}

fn is_valid_player_name<'a>(player_name: &'a [u8; 8]) -> Res<&'a str, &str>{
    match std::str::from_utf8(player_name) {
        Ok(val) => Ok(val),
        Err(_) => Err("invalid town")
    }
}


fn is_valid_town<'a>(town_name: &'a [u8; 7]) -> Res<&'a str, &str>{
    match std::str::from_utf8(town_name) {
        Ok(val) => is_known_town(val),
        Err(_) => Err("invalid town")
    }
}

fn is_known_town(town: &str) -> Res<&str, &str> {
    match TOWN_NAMES.to_vec().contains(&town) {
        true => Ok(town),
        false => Err("invalid town")
    }
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.tx.send(self.out.clone()).unwrap();
        Ok(())
    }
    
    fn on_message(&mut self, _: Message) -> Result<()> {
        Ok(())
    }
}

pub struct Process {
    pub handler: winapi::HANDLE
}

impl Process {
    fn new(m_handler: winapi::HANDLE) -> Process {
        return Process { handler: m_handler }
    }

    pub fn read_memory(&self, addr: &u64, out_ptr: *mut c_void, size: u64) -> bool {
        unsafe {
            let r = kernel32::ReadProcessMemory(self.handler,
                                                *addr as *const _,
                                                out_ptr,
                                                size,
                                                ptr::null_mut()); 
            r == 1
        }
    }
}
pub fn get_proc(pid: u32) -> Process {
    return Process::new(unsafe{kernel32::OpenProcess(winapi::PROCESS_VM_READ, 0, pid)});
}

pub fn get_proc_id_by_name(name: &str) -> u32 {
    
    let mut process: winapi::PROCESSENTRY32W = unsafe{mem::uninitialized()}; 
    process.dwSize = mem::size_of::<winapi::PROCESSENTRY32W>() as u32; 

    //Make a Snanshot of all the current proccess.
    let snapshot = unsafe{kernel32::CreateToolhelp32Snapshot(winapi::TH32CS_SNAPPROCESS, 0)};
    
    //Get the first proccess and store it in proccess variable.
    if unsafe{kernel32::Process32FirstW(snapshot, &mut process)} != 0{
        
        //Take the next procces if posible.
        while unsafe{kernel32::Process32NextW(snapshot, &mut process)} != 0 {    
            
            let process_name = OsString::from_wide(&process.szExeFile);
            
            match process_name.into_string() {
                Ok(s) => {
                    if s.contains(name) {
                        return process.th32ProcessID;
                    }
                },               
                Err(_) => {
                    println!("Error converting process name for PID {}", process.th32ProcessID);
                }          
            }            
        }
    }
    
    return 0;
}

#[derive(Debug, PartialEq)]
pub struct Infos {
    pub materials: Vec<Material>,
    pub town: serde_json::Value,
    pub player: String,
}

impl Infos {
    pub fn new() -> Infos {
        Infos{
            materials: vec![Material::new("test".to_string(), BTreeMap::new()); 20 as usize],
            town: serde_json::Value::String("".to_string()),
            player: "???".to_string()
        }
    }

    pub fn serialize(&self) -> serde_json::Value {
        ObjectBuilder::new() 
            .insert_array("materials", |_| {
                let mut b = ArrayBuilder::new();
                for m in &self.materials {
                    b = b.push(m.serialize());
                }
                b
            })
            .insert("town", &self.town)
            .insert("player", &self.player)
            .unwrap() 
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub name: String,
    pub values: BTreeMap<String, u32>
}

impl Material {
    pub fn new(name: String, values: BTreeMap<String, u32>) -> Material {
        Material{name: name, values: values}
    }

    pub fn serialize(&self) -> serde_json::Value {
        ObjectBuilder::new()
            .insert("name", self.name.clone()) 
            .insert("values", &self.values)
            .unwrap()
    }

    pub fn is_empty(&self) -> bool {
        for (_, &val) in &self.values{
            if val != 0 {
                return false;
            } 
        }
        true
    }
}

// impl PartialEq for Material {
//     pub fn eq(&self, material: &Material) -> bool {
    
//     } 
// }
