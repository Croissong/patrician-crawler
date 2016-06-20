use std::{ptr, mem};
use std::os::raw::c_void;
use std::ffi::OsString;
use super::{winapi, kernel32};

use wio::wide::FromWide;

pub fn get_proc_by_name(name: &str) -> Result<Process, &str> {
    let pid = get_proc_id_by_name(name);
    match pid {
        0 => Err("No Patrician process found."),
        pid => Ok(get_proc(pid))
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
fn get_proc(pid: u32) -> Process {
    return Process::new(unsafe{kernel32::OpenProcess(winapi::PROCESS_VM_READ, 0, pid)});
}

fn get_proc_id_by_name(name: &str) -> u32 {
    
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
