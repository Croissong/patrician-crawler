#![feature(custom_derive, plugin)]
#![plugin(serde_macros, clippy)]
#![allow(needless_lifetimes, identity_op, clone_on_copy, clone_double_ref)]
#[macro_use] extern crate log;

extern crate rand;
extern crate kernel32;
extern crate winapi;
extern crate wio;
extern crate time;
extern crate serde;
extern crate serde_json;
extern crate ws;
extern crate env_logger;

mod crawler;

use std::time::Duration;
use std::thread;
use crawler::websocket::{spawn_websocket, send_infos};
use crawler::crawler::{Crawler};
use crawler::process::{Process, get_proc_by_name};

fn main() {
    env_logger::init().unwrap();
    match get_proc_by_name("Patrician3.exe") {
        Err(err) => println!("{}", err),
        Ok(process) => start_crawler(process)
    }
}

fn start_crawler(process: Process) {
    println!("Found Patrician process");
    let socket = spawn_websocket(); 
    let mut crawler = Crawler::new(process); 
    loop {
        match crawler.crawl() {
            Ok(infos) => send_infos(&infos, &socket),
            Err(err) => println!("{}", err) 
        }
        thread::sleep(Duration::from_millis(1000));
    }
}
