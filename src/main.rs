#![feature(custom_derive, plugin, try_from)]
#![plugin(serde_macros, clippy)]
#![allow(identity_op)]
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
extern crate getopts;

mod crawler;
mod test_crawler;

use std::time::Duration;
use std::thread;
use crawler::{ websocket, process, mock, constants };
use getopts::{Options, Matches};

fn main() {
    env_logger::init().unwrap();
    let args = get_cli_args(); 
    if is_dev_mode(&args) {
        println!("Starting crawler in dev mode");
        mock::mock();
    } else {
        match unsafe{process::get_proc_by_name("Patrician3.exe")} {
            Ok(process) => {
                let host = get_server_host(&args);
                start_crawler(process, host)
            },
            Err(err) => println!("{}", err)
        }
    }
}

fn start_crawler(process: process::Process, host: String) {
    if let Some(socket) = websocket:: spawn_websocket(host) {
        let ( mut crawler, inital_infos ) = crawler::crawler::Crawler::new(process); 
        loop {
            if let Some(infos) = crawler.crawl(&inital_infos) {
                websocket::send_infos(&infos, &socket);
            }
            thread::sleep(Duration::from_millis(1000));
        }
    }
}

fn get_cli_args() -> Matches {
    let args: Vec<String> = std::env::args().collect(); 
    let mut opts = Options::new();
    opts.optopt("s", "", "set server host", "SERVER_HOST");
    opts.optflag("d", "dev", "enable dev mode");
    match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    } 
}

fn get_server_host(args: &Matches) -> String {
    if let Some(host) = args.opt_str("s") {
        format!("ws://{}/socket/websocket?vsn=1.0.0", host)
    } else {
        constants::SERVER_URL.to_string()
    }
}

fn is_dev_mode(args: &Matches) -> bool {
    args.opt_present("dev")
}
