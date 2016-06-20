use ws::{connect, Handler, Sender, Handshake, Result, Message};
use std::sync::mpsc::channel;
use std::thread;
use std::sync::mpsc;
use structs::{Infos};
use time;
use serde_json;

pub fn spawn_websocket() -> Sender {
    let (tx, rx) = channel(); 
    thread::spawn(move|| {
        connect("ws://localhost:4000/socket/websocket?vsn=1.0.0", |out| {
            Client { out: out, tx: tx.clone() }
        }).unwrap();             // send the Sender to thread 1

    });
    let socket: Sender = rx.recv().unwrap();
    join_channel(&socket);
    socket
}

fn join_channel(out: &Sender){
    let str = "{\"topic\":\"rust_client:web_client\",\"ref\":\"1\",\"payload\":{},\"event\":\"phx_join\"}"; 
    out.send(str).unwrap();
}

pub fn send_infos(infos: &Infos, socket: &Sender) {
    let json = serde_json::to_string(&infos).unwrap();
    println!("{:?}", json);
    let string = format!("{{\"topic\":\"rust_client:web_client\",\"ref\":null,\"payload\":{{\"body\":{}}},\"event\":\"rust_client:web_client\"}}", json);
    println!("sent update at {}", time::now().ctime());
    socket.send(string).unwrap();
}

struct Client {
    pub out: Sender,
    pub tx: mpsc::Sender<Sender>        
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
