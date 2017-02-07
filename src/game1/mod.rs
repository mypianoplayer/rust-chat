extern crate mio_websocket;
extern crate env_logger;
extern crate mio;

mod entity_component;

use std::net::SocketAddr;
use std::sync::{Arc,Weak,mpsc};
use std::cell::RefCell;
use std::{thread, time};
use self::mio_websocket::interface::*;
use self::entity_component::*;

pub struct Game {
    system : Arc<RefCell<System>>,
    server : Arc<RefCell<WebSocket>>,
}

impl Game {

    pub fn new(host_addr: &str) -> Game {
        Game {
            system : Arc::new(RefCell::new(System::new())),
            server : Arc::new(RefCell::new(WebSocket::new(host_addr.parse::<SocketAddr>().unwrap()))),
        }
    }

    fn onmessage(&self, tok:mio::Token, msg: &String) {
        println!("{}", msg);
        if msg.eq("start") {
            let mut obj = Entity::new();
            {
                let cmp = InputComponent::new(tok);
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
            self.server.borrow_mut().send_all("script setup_button(1,false,'')".to_string());
            self.server.borrow_mut().send_all("script setup_button(2,false,'')".to_string());
            self.server.borrow_mut().send_all("script setup_button(3,false,'')".to_string());
        }
        if msg.eq("end") {
            let mut remove_id = 0;
            for e in self.system.borrow().entities() {
                let ent = e.borrow();
                let inp = ent.component(1);
                if let Component::Input(ref input) = *inp {
                   if input.token().eq(&tok) {
                       remove_id = ent.id();
                   }
                }
            }
            self.system.borrow_mut().disable_entity(remove_id);
        }
        if msg.starts_with("click") {
            let mut it = msg.split_whitespace();
            it.next();
            let x :f32 = it.next().unwrap().parse().unwrap();
            let y :f32 = it.next().unwrap().parse().unwrap();
            for e in self.system.borrow_mut().entities_mut() {
                let mut ent = e.borrow_mut();
                let mut inp = ent.component_mut(1);
                if let Component::Input(ref mut input) = *inp {
                   if input.token().eq(&tok) {
                        input.set_clicked_pos((x,y));
                   }
                }

            }

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

                        (tok, WebSocketEvent::TextMessage(msg)) => {
                            self.onmessage(tok, &msg);
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
                            thread::sleep( time::Duration::from_millis(33) );
                            // self.server.borrow_mut().send_all("script clear_screen()".to_string());
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
