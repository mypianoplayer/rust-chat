
extern crate mio_websocket;
extern crate env_logger;
extern crate rust_chat;

use std::net::SocketAddr;
use std::sync::mpsc;
use std::{thread, time};
use mio_websocket::interface::*;
use rust_chat::entity_component;

fn main() {
    env_logger::init().unwrap();

    let mut game = Game1::new("0.0.0.0:10000");

    game.start();

}


struct Game1<'a> {
    system : entity_component::System<'a>,
    server : WebSocket

}

impl<'a> Game1<'a> {

    fn new(host_addr: &str) -> Game1 {
        Game1 {
            system : entity_component::System::new(),
            server : WebSocket::new(host_addr.parse::<SocketAddr>().unwrap())
        }
    }

    fn onmessage(&mut self, msg: &String) {
        println!("{}", msg);
    }

    fn update(&mut self) {
       for peer in self.server.get_connected().unwrap() {
           let response = WebSocketEvent::TextMessage("tick".to_string());
           self.server.send((peer, response));
       }
    }

    fn start(&mut self) {
        loop {
            match self.server.try_next() {
                Ok(r) => {
                    match r {
                        (tok, WebSocketEvent::Connect) => {
                            println!("connected peer: {:?}", tok);
                        },

                        (tok, WebSocketEvent::TextMessage(msg)) => {
                            self.onmessage(&msg);
                        },

                        (tok, WebSocketEvent::BinaryMessage(msg)) => {
                            println!("msg from {:?}", tok);
                            let response = WebSocketEvent::BinaryMessage(msg.clone());
                            self.server.send((tok, response));
                        },

                        _ => {}
                    }
                },
                Err(e) => {
                    match e {
                        mpsc::TryRecvError::Empty => {
                            thread::sleep( time::Duration::from_millis(1000) );
                            self.update();
                        },
                        mpsc::TryRecvError::Disconnected => {

                        }
                    }
                }

            }
        }
    }
}
