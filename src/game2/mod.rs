extern crate mio_websocket;
extern crate env_logger;

mod entity_component;

use std::net::SocketAddr;
use std::sync::{Arc,Weak,mpsc};
use std::cell::RefCell;
use std::{thread, time};
use self::mio_websocket::interface::*;
use self::entity_component::*;

pub struct Game {
    system : Arc<RefCell<System>>,
    server : Arc<RefCell<WebSocket>>
}

impl Game {

    pub fn new(host_addr: &str) -> Game {
        Game {
            system : Arc::new(RefCell::new(System::new())),
            server : Arc::new(RefCell::new(WebSocket::new(host_addr.parse::<SocketAddr>().unwrap()))),
        }
    }

    fn onmessage(&self, msg: &String) {
        println!("{}", msg);
        if msg.eq("start") {
            let mut obj = Entity::new();
            {
                let cmp = InputComponent::new();
                obj.add_component(Component::Input(cmp));
            }
            {
                let cmp = PositionComponent::new();
                obj.add_component(Component::Position(cmp));
            }
            {
                let cmp = ObjectViewComponent::new(Arc::downgrade(&self.server));
                obj.add_component(Component::ObjectView(cmp));
            }
            self.system.borrow_mut().add_entity(obj);
        }
    }

    // fn send_all(&mut self, msg: String) {
    //     for peer in self.server.borrow_mut().get_connected().unwrap() {
    //         let evt = WebSocketEvent::TextMessage(msg.clone());
    //         self.server.borrow_mut().send((peer, evt));
    //     }
    // }

    // fn update(&mut self) {
    //     self.system.borrow_mut().update();
    //    for peer in self.server.borrow_mut().get_connected().unwrap() {
    //        let response = WebSocketEvent::TextMessage("tick".to_string());
    //        self.server.borrow_mut().send((peer, response));
    //    }
    // }

    pub fn start(&mut self) {
        loop {
            let res = self.server.borrow_mut().try_next();
            match res {
                Ok(r) => {
                    match r {
                        (tok, WebSocketEvent::Connect) => {
                            println!("connected peer: {:?}", tok);
                        },

                        (_, WebSocketEvent::TextMessage(msg)) => {
                            self.onmessage(&msg);
                        },

                        (tok, WebSocketEvent::BinaryMessage(msg)) => {
                            println!("msg from {:?}", tok);
                            let response = WebSocketEvent::BinaryMessage(msg.clone());
                            self.server.borrow_mut().send((tok, response));
                        },

                        _ => {}
                    }
                },
                Err(e) => {
                    match e {
                        mpsc::TryRecvError::Empty => {
                            thread::sleep( time::Duration::from_millis(1000) );
                            self.system.borrow_mut().update();
                        },
                        mpsc::TryRecvError::Disconnected => {

                        }
                    }
                }

            }
        }
    }
}
