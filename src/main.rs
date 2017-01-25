
extern crate mio_websocket;
extern crate env_logger;

use std::net::SocketAddr;
use std::sync::mpsc;
use std::{thread, time};
use mio_websocket::interface::*;

fn main() {
    env_logger::init().unwrap();

    let mut ws = WebSocket::new("0.0.0.0:10000".parse::<SocketAddr>().unwrap());

    loop {
        match ws.try_next() {
            Ok(r) => {
                match r {
                    (tok, WebSocketEvent::Connect) => {
                        println!("connected peer: {:?}", tok);
                    },

                    (tok, WebSocketEvent::TextMessage(msg)) => {
                        println!("{:?}", msg);
                        for peer in ws.get_connected().unwrap() {
                            if peer != tok {
                                println!("-> relaying to peer {:?}", peer);

                                let response = WebSocketEvent::TextMessage(msg.clone());
                                ws.send((peer, response));
                            }
                        }
                    },

                    (tok, WebSocketEvent::BinaryMessage(msg)) => {
                        println!("msg from {:?}", tok);
                        let response = WebSocketEvent::BinaryMessage(msg.clone());
                        ws.send((tok, response));
                    },

                    _ => {}
                }
            },
            Err(e) => {
                match e {
                    mpsc::TryRecvError::Empty => {
                        for peer in ws.get_connected().unwrap() {
                            let response = WebSocketEvent::TextMessage("tick".to_string());
                            ws.send((peer, response));
                        }
                        thread::sleep( time::Duration::from_millis(1000) );
                    },
                    mpsc::TryRecvError::Disconnected => {

                    }
                }
            }

        }
    }
}
