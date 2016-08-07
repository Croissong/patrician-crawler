use std::{ptr, mem};
use std::os::raw::c_void;
use std::ffi::OsString;
use std::convert::TryFrom;
use super::super::{winapi, kernel32};

use wio::wide::FromWide;

pub unsafe fn get_proc_by_name(name: &str) -> Result<Process, &str> {
    let pid = get_proc_id_by_name(name);
    match pid {
        0 => Err("No Patrician process found."),
        pid => {
            println!("Found Patrician process with id {}", pid);
            Ok(get_proc(pid))
        }
    } 
}

#[derive(Debug)]
pub struct Pointer {
    pub addr: u64,
    pub offsets: Vec<i64>
}

#[derive(Clone, Copy)]
pub struct Process {
    pub handler: winapi::HANDLE
}

impl Process {
    fn new(m_handler: winapi::HANDLE) -> Process {
        Process { handler: m_handler }
    }

    pub fn read_ptr(&self, ptr: &Pointer) -> u64 {
        let mut result: u64 = 0;
        let mut reading_addr: u64 = ptr.addr;
        for offset in &ptr.offsets {
            self.read_memory(&reading_addr, &mut result as *mut _ as *mut _, 4);
            result += u64::try_from(offset.to_owned()).unwrap();
            reading_addr = result;
        }
        result
    }


    pub fn read_memory(&self, addr: &u64, out_ptr: *mut c_void, size: u64) -> bool {
        unsafe{
            let r = kernel32::ReadProcessMemory(self.handler,
                                                *addr as *const _,
                                                out_ptr,
                                                size,
                                                ptr::null_mut());
            if r == 0 {
                read_memory_err();
            }
            r == 1 
        }
    }
}

unsafe fn read_memory_err() {
    let err = kernel32::GetLastError();
    if err == 6 {
        println!("Start with administrator privileges.");
    } else {
        println!("{}", err);
    }
}

unsafe fn get_proc(pid: u32) -> Process {
    Process::new(kernel32::OpenProcess(winapi::PROCESS_VM_READ, 0, pid))
}

unsafe fn get_proc_id_by_name(name: &str) -> u32 {
    let mut process: winapi::PROCESSENTRY32W = mem::uninitialized();
    process.dwSize = mem::size_of::<winapi::PROCESSENTRY32W>() as u32; 
    //Make a Snanshot of all the current proccess.
    let snapshot = kernel32::CreateToolhelp32Snapshot(winapi::TH32CS_SNAPPROCESS, 0); 
    //Get the first proccess and store it in proccess variable.
    if kernel32::Process32FirstW(snapshot, &mut process) != 0{
        //Take the next procces if posible.
        while kernel32::Process32NextW(snapshot, &mut process) != 0 {
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
    0
}
