#![feature(custom_derive, plugin)]
#![plugin(serde_macros, clippy)]
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
use websocket::{spawn_websocket, send_infos};
use crawler::{Crawler, crawl};
use process::{Process};

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
    let crawler = Crawler::new(socket, addresses, process); 
    loop {
        match crawler.crawl() {
            Ok(infos) => send_infos(&infos, &socket),
            Err(err) => println!("{}", err) 
        }
        thread::sleep(Duration::from_millis(1000));
    }
}
