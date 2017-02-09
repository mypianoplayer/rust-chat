extern crate mio_websocket;
extern crate mio;

use std::cell::*;
use std::sync::{Arc,Weak};
use std::collections::HashSet;
use game2::entity_component::*;
use game2::mio_websocket::interface::WebSocket;

pub struct ObjectViewComponent {
    server: Weak<RefCell<WebSocket>>,
    name: String,
    prev_hp: i32,
    tokens: HashSet<usize>
}

impl ObjectViewComponent {
    pub fn new(sv: Weak<RefCell<WebSocket>>, name:String) -> ObjectViewComponent {
        ObjectViewComponent {
            server:sv,
            name:name,
            prev_hp:0,
            tokens:HashSet::new(),
        }
    }
    pub fn update(&mut self, parent:&Entity) {
        // println!("objectview!!");

        if let Some(sv) = self.server.upgrade() {

            let mut s = sv.borrow_mut();
            for peer in s.get_connected().unwrap() {
                let tok = peer.0;
                if !self.tokens.contains(&tok) {
                    self.tokens.insert(tok);
                    let txt = format!("{}/HP:{}", self.name, 100);
                    let cmd = format!("script new_object('{}','{}')", self.name, txt);
                    s.send_peer(tok, cmd);

                    // if let Component::Position(ref pos) = *parent.component(2) {
                    //     let position = pos.pos();
                    //     let cmd = format!("script object_set_pos('{}',{},{});", self.name, position.0, position.1 );
                    //     s.send_all(cmd);
                    // }
                }
                if let Component::BattleStatus(ref bat) = *parent.component(3) {
                    let hp = bat.hp();
                    if hp != self.prev_hp {
                        self.prev_hp = hp;
                        let txt = format!("{}/HP:{}", self.name, hp);
                        let cmd = format!("script object_set_text('{}','{}')", self.name, txt);
                        s.send_all(cmd);
                    }
                }
            }

        }
    }
    pub fn new_object(&self, tok: usize) {
        if let Some(sv) = self.server.upgrade() {
            let cmd = format!("script new_object('{}',{})", self.name, "init");
            sv.borrow_mut().send_peer(tok, cmd);
        }
    }
}
