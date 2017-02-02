
extern crate mio_websocket;
extern crate env_logger;
extern crate rust_chat;

use std::net::SocketAddr;
use std::sync::{Arc,Weak,mpsc};
use std::cell::RefCell;
use std::{thread, time};
use mio_websocket::interface::*;
use rust_chat::entity_component;

enum ComponentTypeId {
    Input,
    Position,
    ObjectView,
}

fn main() {
    env_logger::init().unwrap();

    let mut game = Game1::new("0.0.0.0:10000");

    game.start();

}

struct InputComponent {
    clicked_pos: (f32,f32)
}

impl InputComponent {
    fn set_clicked_pos(&mut self,pos:(f32,f32)) {
        self.clicked_pos = pos;
    }
    fn clicked_pos(&self) -> (f32,f32) {
        self.clicked_pos
    }
}

impl entity_component::Component for InputComponent {
    fn update(&mut self, parent:&entity_component::Entity) {

    }
    fn typeid(&self) -> i32 {
        ComponentTypeId::Input as i32
    }
}

struct PositionComponent {
    pos: (f32,f32),
    input: Weak<RefCell<InputComponent>>
}

impl PositionComponent {
    fn pos(&self) -> (f32,f32) {
        self.pos
    }
}

impl entity_component::Component for PositionComponent {
    fn update(&mut self, parent:&entity_component::Entity) {
        if let Some(input) = self.input.upgrade() {
            let tgtpos = input.borrow().clicked_pos();
            self.pos.0 = self.pos.0 + (tgtpos.0 - self.pos.0) * 0.05;
            self.pos.1 = self.pos.1 + (tgtpos.1 - self.pos.1) * 0.05;
        }
    }
    fn typeid(&self) -> i32 {
        ComponentTypeId::Position as i32
    }
}

struct ObjectViewComponent {
    server: Weak<RefCell<WebSocket>>,
    pos: Weak<RefCell<PositionComponent>>
}

impl ObjectViewComponent {
    fn new(sv: Weak<RefCell<WebSocket>>) -> ObjectViewComponent {
        ObjectViewComponent {
            server:sv,
            pos:Weak::new(),
        }
    }
}

impl entity_component::Component for ObjectViewComponent {
    fn update(&mut self, parent:&entity_component::Entity) {
        if let Some(sv) = self.server.upgrade() {
            // let pos = parent.component(ComponentTypeId::Position as i32);
            // let p = pos.borrow().pos();
            sv.borrow_mut().send_all("script draw_circle(10,10,50)".to_string());
        }
    }
    fn typeid(&self) -> i32 {
        ComponentTypeId::ObjectView as i32
    }
}

struct Game1 {
    system : Arc<RefCell<entity_component::System>>,
    server : Arc<RefCell<WebSocket>>
}

impl Game1 {

    fn new(host_addr: &str) -> Game1 {
        Game1 {
            system : Arc::new(RefCell::new(entity_component::System::new())),
            server : Arc::new(RefCell::new(WebSocket::new(host_addr.parse::<SocketAddr>().unwrap()))),
        }
    }

    fn onmessage(&self, msg: &String) {
        println!("{}", msg);
        if msg.eq("start") {
            let mut obj = entity_component::Entity::new();
            let cmp = ObjectViewComponent::new(Arc::downgrade(&self.server));
            obj.add_component(Arc::new(RefCell::new(cmp)));
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

    fn start(&mut self) {
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
