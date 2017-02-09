extern crate mio_websocket;
extern crate env_logger;
extern crate mio;
extern crate rand;

mod entity_component;
mod battle_status_component;
mod object_view_component;
mod position_component;

use std::net::SocketAddr;
use std::sync::{Arc,Weak,mpsc};
use std::cell::*;
use std::{thread, time};
use self::mio_websocket::interface::*;
use self::entity_component::*;

use self::battle_status_component::*;
use self::position_component::*;
use self::object_view_component::*;

pub struct Game {
    system : Arc<RefCell<System>>,
    server : Arc<RefCell<WebSocket>>,
//    rnd : Box<rand::Rng>,
}

impl Game {

    pub fn new(host_addr: &str) -> Game {
        Game {
            system : Arc::new(RefCell::new(System::new())),
            server : Arc::new(RefCell::new(WebSocket::new(host_addr.parse::<SocketAddr>().unwrap()))),
//            rnd : Box::new(rand::thread_rng()),
        }
    }

    fn onmessage(&self, tok:mio::Token, msg: &String) {
        println!("{}", msg);
        if msg.eq("start") {
            let mut obj = Entity::new();
            {
                obj.set_param("player_id".to_string(), tok.as_usize() as i32);
            }
            {
                let mut cmp = PositionComponent::new();
                cmp.set_pos((10.0,10.0));
                obj.add_component(Component::Position(cmp));
            }
            {
                let cmp = BattleStatusComponent::new();
                obj.add_component(Component::BattleStatus(cmp));
            }
            {
                let cmp = ObjectViewComponent::new(Arc::downgrade(&self.server), format!("PL{}", tok.as_usize()));
                obj.add_component(Component::ObjectView(cmp));
            }
            self.system.borrow_mut().add_entity(obj);
            // self.server.borrow_mut().send_peer(tok.as_usize(), "script clear_ui()".to_string());
            // self.server.borrow_mut().send_peer(tok.as_usize(), "script setup_screen(false)".to_string());
            self.server.borrow_mut().send_peer(tok.as_usize(), "script setup_button(1,true,'攻撃')".to_string());
            self.server.borrow_mut().send_peer(tok.as_usize(), "script setup_button(2,false,'')".to_string());
            self.server.borrow_mut().send_peer(tok.as_usize(), "script setup_button(3,false,'')".to_string());
        }
        if msg.eq("button1") {
            let mut sys = self.system.borrow_mut();
            let mut en_me : Option<Arc<RefCell<Entity>>> = None;
            let mut en_tgt : Option<Arc<RefCell<Entity>>> = None;
            {
                let mut entities = sys.entities_mut();
                let tgt_idx = rand::random::<u32>() % entities.len() as u32;
                println!("idx {}", tgt_idx);
                let mut i = 0;
                for en in entities.iter_mut() {
                    if i == tgt_idx {
                        en_tgt = Some(en.clone());
                    }
                    if en.borrow_mut().param("player_id".to_string()) == tok.as_usize() as i32 {
                        en_me = Some(en.clone());
                    }
                    i += 1;
                }
            }
            let mut btl_me = en_me.unwrap();
            let mut btl_tgt = en_tgt.unwrap();
            let id_me = btl_me.borrow().id();
            let id_tgt = btl_tgt.borrow().id();
            if id_me == id_tgt {
                let text = format!("{}の攻撃!効果がなかった", id_me);
                let cmd = format!("script show_text('{}')", text);
                self.server.borrow_mut().send_all(cmd);
            } else {
                let mut btl_me = btl_me.borrow_mut();
                let mut btl_me = btl_me.component_mut(3);
                let mut btl_tgt = btl_tgt.borrow_mut();
                let mut btl_tgt = btl_tgt.component_mut(3);
                if let Component::BattleStatus(ref mut btl_me) = *btl_me {
                    if let Component::BattleStatus(ref mut btl_tgt) = *btl_tgt {
                        let damage = btl_tgt.attacked(&btl_me);
                        println!("attack {} -> {}", id_me, id_tgt);
                        let text = format!("{}の攻撃!>>{}に{}のダメージ!", id_me, id_tgt, damage);
                        let cmd = format!("script show_text('{}')", text);
                        self.server.borrow_mut().send_all(cmd);
                    }
                }
            }
        }
        if msg.eq("end") {
            let mut remove_id : Option<i32> = None;
            {
                let sys = self.system.borrow();
                let entities = sys.entities();
                for en in entities.iter() {
                    if en.borrow().param("player_id".to_string()) == tok.as_usize() as i32 {
                        remove_id = Some(en.borrow().id());
                    }
                }
            }
            if let Some(remove_id) = remove_id {
                self.system.borrow_mut().disable_entity(remove_id);
            }
        }
        if msg.starts_with("click") {
            // let mut it = msg.split_whitespace();
            // it.next();
            // let x :f32 = it.next().unwrap().parse().unwrap();
            // let y :f32 = it.next().unwrap().parse().unwrap();
            // for e in self.system.borrow_mut().entities_mut() {
            //     let mut ent = e.borrow_mut();
            //     let mut inp = ent.component_mut(1);
            //     if let Component::Input(ref mut input) = *inp {
            //        if input.token().eq(&tok) {
            //             input.set_clicked_pos((x,y));
            //        }
            //     }
            //
            // }

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
