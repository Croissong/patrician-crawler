use ws::{connect, Handler, Sender, Handshake, Result as Res,
         Message, CloseCode, Error, util};
use std::sync::mpsc::channel;
use std::thread;
use std::sync::mpsc;
use super::structs::{Infos};
use super::constants;
use time;
use serde_json;

pub fn spawn_websocket(server_url: String) -> Option<Sender> {
    let (tx, rx) = channel(); 
    thread::spawn(move|| {
        if let Err(error) = connect(server_url.clone().as_str(), |out| {
            out.timeout(2000, out.token()).unwrap();
            Client { out: out, tx: tx.clone() , host: server_url.clone()}
        }) {
            // Inform the user of failure
            println!("Failed to create WebSocket due to: {:?}", error);
        }

    });
    match rx.recv().unwrap() {
        Some(socket) => {
            join_channel(&socket);
            Some(socket)
        }
        None => None
    }
}

fn join_channel(out: &Sender){
    println!("joined channel");
    out.send(constants::CHANNEL).unwrap();
}

pub fn send_infos(infos: &Infos, socket: &Sender) {
    let json = serde_json::to_string(&infos).unwrap(); 
    let string = format!("{{\"topic\":\"rust_client:web_client\",\"ref\":null,\"payload\":{{\"body\":{}}},\"event\":\"rust_client:web_client\"}}", json);
    println!("sent update at {}", time::now().ctime());
    socket.send(string).unwrap();
}

struct Client {
    pub host: String,
    pub out: Sender,
    pub tx: mpsc::Sender<Option<Sender>>
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Res<()> {
        println!("opened websocket connection to {} successfully", &self.host);
        self.tx.send(Some(self.out.clone())).unwrap(); 
        Ok(())
    }
    
    fn on_message(&mut self, _: Message) -> Res<()> {
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
    
    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }

    fn on_timeout(&mut self, _: util::Token) -> Res<()> {
        println!("Unable to connect to websocket server {}", self.host);
        self.tx.send(None).unwrap(); 
        Ok(())
    }
}
