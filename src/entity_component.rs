use std::borrow::Borrow;
use std::cell::RefCell;
use std::slice;
use std::sync::Arc;

pub trait Component {
    fn update(&mut self);
    fn typeid(&self) -> i32;
}

pub struct Entity {
    pub comps : Vec<Arc<RefCell<Component>>>
}

impl Entity {
    pub fn add_component(& mut self, comp : Arc<RefCell<Component>> ) {
        self.comps.push( comp );
    }
    pub fn components(&self) -> &Vec<Arc<RefCell<Component>>> {
        &self.comps
    }
}

pub struct System {
    comps : Vec<Arc<RefCell<Component>>>,
    objects : Vec<Box<Entity>>
}

impl System {
    pub fn new() -> System {
        System {
            comps : Vec::new(),
            objects : Vec::new()
        }
    }
    pub fn add_entity(& mut self, entity:Entity) {
        for c in entity.components() {
            self.comps.push(c.clone());
        }
        self.objects.push(Box::new(entity));
    }
    pub fn update(&mut self) {
        for c in &self.comps {
            c.borrow_mut().update();
        }
    }
}
