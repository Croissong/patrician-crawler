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
use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;


struct Client {
    out: Sender,
    tx: std::sync::mpsc::Sender<Sender>
    
}

fn main() {
    let (tx, rx) = channel(); 

    thread::spawn(move|| {
        connect("ws://echo.websocket.org/", |out| {
            Client { out: out, tx: tx.clone() }
        }).unwrap();             // send the Sender to thread 1

    });
    
    let out: ws::Sender = rx.recv().unwrap(); 

    const MATERIALS: [&'static str; 20] = ["grain", "meat", "fish", "beer", "salt", "honey", "spices", "wine", "cloth", "skins", "whale oil", "timber", "iron goods", "leather", "wool", "pitch", "pig iron", "hemp", "pottery", "bricks"];

    let pid = get_proc_id_by_name("Patrician3.exe"); 
    println!("Found process with id {}", &pid);
    let address: u64 = 0x006D9584;
    let mut pointer: u64 = 0;
    get_proc(pid).read_memory(&address,
                              &mut pointer as *mut _ as *mut _,
                              4);
    pointer += 0x524;
    let mut block = [0u32; 110 as usize];
    let mut material_map = BTreeMap::new();
    for i in 0..MATERIALS.len() {
        material_map.insert(MATERIALS[i], [0u32; 5 as usize]);
    }
    loop {        
        get_proc(pid).read_memory (&pointer, &mut block as *mut _ as *mut _, 440);
        let mut dirty_flag = false;
        for i in 0..MATERIALS.len() {
            let val = [block[i], block[i+50], block[i+70], block[i+25], block[i+90]];
            if material_map.insert(MATERIALS[i], val).unwrap() != val {
                dirty_flag = true;    
            }
        }
        if dirty_flag {
            material_map.insert("town", [block[24], block[50], 0u32, 0u32, 0u32]);
            let json = serde_json::to_string(&material_map).unwrap(); 
            out.send(json.as_str()).unwrap();
        } 
        thread::sleep(Duration::from_millis(1000));
    }
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let res = self.out.send("Hello WebSocket");
        self.tx.send(self.out.clone()).unwrap();
        res
    }
    
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Got message: {}", msg);
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
