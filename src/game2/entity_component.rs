extern crate mio_websocket;
extern crate mio;

use std::cell::*;
use std::sync::{Arc,Weak};
use std::collections::hash_map::HashMap;
use std::collections::btree_set::BTreeSet;
use self::mio_websocket::interface::*;
use game2::battle_status_component::BattleStatusComponent;
use game2::object_view_component::ObjectViewComponent;
use game2::position_component::PositionComponent;

pub enum Component {
    Input(InputComponent),
    Position(PositionComponent),
    BattleStatus(BattleStatusComponent),
    ObjectView(ObjectViewComponent),
}

impl Component {
    pub fn update(&mut self, parent:&Entity ) {
        match *self {
            Component::Input(ref mut v) => {
                v.update(parent);
            },
            Component::Position(ref mut v) => {
                v.update(parent);
            },
            Component::BattleStatus(ref mut v) => {
                v.update(parent);
            }
            Component::ObjectView(ref mut v) => {
                v.update(parent);
            }
        }
    }
    pub fn type_id(&self) -> i32 {
        match *self {
            Component::Input(_) => {
                1
            },
            Component::Position(_) => {
                2
            },
            Component::BattleStatus(_) => {
                3
            }
            Component::ObjectView(_) => {
                4
            }
        }
    }
}

pub struct InputComponent {
    token: mio::Token,
    clicked_pos: (f32,f32),
}

impl InputComponent {
    pub fn new(tok: mio::Token) -> InputComponent {
        InputComponent{
            token: tok,
            clicked_pos:(0.0,0.0),
        }
    }
    pub fn set_clicked_pos(&mut self,pos:(f32,f32)) {
        self.clicked_pos = pos;
    }
    pub fn clicked_pos(&self) -> (f32,f32) {
        self.clicked_pos
    }
    pub fn token(&self) -> &mio::Token {
        &self.token
    }

    pub fn update(&mut self, parent:&Entity) {
        // println!("input!!")
    }
}


pub struct Entity {
    pub comps : HashMap<i32,RefCell<Component>>,
    params: HashMap<String,i32>,
    id : i32,
    enable : bool,
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            comps: HashMap::new(),
            params: HashMap::new(),
            id:0,
            enable: true,
        }
    }
    pub fn set_entity_id(&mut self,id:i32) {
        self.id = id;
    }
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn param(&self, key:String) -> i32 {
        *self.params.get(&key).unwrap()
    }
    pub fn set_param(&mut self, key:String, value:i32) {
        self.params.insert(key, value);
    }
    pub fn add_component(&mut self, comp:Component ) {
        let id = comp.type_id();
        self.comps.insert(id, RefCell::new(comp));
    }
    pub fn update_entity(&mut self, component_type_id:i32) {
        if self.enable == false { return; }
        if let Some(comp) = self.comps.get(&component_type_id) {
            comp.borrow_mut().update(&self);
        }
    }
    pub fn component(&self, type_id:i32) -> Ref<Component> {
        self.comps.get(&type_id).unwrap().borrow()
    }
    pub fn component_mut(&mut self, type_id:i32) -> RefMut<Component> {
        self.comps.get(&type_id).unwrap().borrow_mut()
    }
    pub fn components(&self) -> &HashMap<i32,RefCell<Component>> {
        &self.comps
    }
    pub fn disable(&mut self) {
        self.enable = false;
    }
}

pub struct System {
    // comps : Vec<Weak<RefCell<Component>>>,
    objects : Vec<Arc<RefCell<Entity>>>,
    component_type_ids : BTreeSet<i32>,
    entity_id_max : i32,
}

impl System {
    pub fn new() -> System {
        System {
            // comps : Vec::new(),
            objects: Vec::new(),
            component_type_ids: BTreeSet::new(),
            entity_id_max:0,
        }
    }
    pub fn add_entity(&mut self, mut entity:Entity) {
        for (id,_) in entity.components() {
            self.component_type_ids.insert(*id);
        }
        entity.set_entity_id(self.entity_id_max);
        self.objects.push(Arc::new(RefCell::new(entity)));
        self.entity_id_max += 1;
    }
    pub fn entities(&self) -> &Vec<Arc<RefCell<Entity>>> {
        &self.objects
    }
    pub fn entities_mut(&mut self) -> &mut Vec<Arc<RefCell<Entity>>> {
        &mut self.objects
    }
    pub fn disable_entity(&mut self, entity_id:i32) {
        for e in &mut self.objects {
            let mut ent = e.borrow_mut();
            if ent.id() == entity_id {
                ent.disable();
            }
        }
    }
    pub fn update(&mut self) {
        for id in &self.component_type_ids {
            for o in &self.objects {
                o.borrow_mut().update_entity(*id);
            }
        }
    }
}
