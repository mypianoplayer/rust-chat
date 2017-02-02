use std::cell::RefCell;
use std::sync::{Arc,Weak};
use std::collections::hash_map::HashMap;
use std::collections::btree_set::BTreeSet;

pub trait Component {
    fn update(&mut self, parent:&Entity);
    fn typeid(&self) -> i32;
}

pub struct Entity {
    pub comps : HashMap<i32,Arc<RefCell<Component>>>
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            comps: HashMap::new()
        }
    }
    pub fn add_component(&mut self, comp: Arc<RefCell<Component>> ) {
        let id = comp.borrow().typeid();
        self.comps.insert(id, comp);
    }
    pub fn update_entity(&self, component_type_id:&i32) {
        if let Some(comp) = self.comps.get(component_type_id) {
            comp.borrow_mut().update(&self);
        }
    }
    pub fn component(&self, type_id:i32) -> &Arc<RefCell<Component>> {
        self.comps.get(&type_id).unwrap()
    }
    pub fn components(&self) -> &HashMap<i32,Arc<RefCell<Component>>> {
        &self.comps
    }
}

pub struct System {
    // comps : Vec<Weak<RefCell<Component>>>,
    objects : Vec<Box<Entity>>,
    component_type_ids : BTreeSet<i32>
}

impl System {
    pub fn new() -> System {
        System {
            // comps : Vec::new(),
            objects: Vec::new(),
            component_type_ids: BTreeSet::new(),
        }
    }
    pub fn add_entity(&mut self, entity:Entity) {
        for (id,_) in entity.components() {
            self.component_type_ids.insert(*id);
        }
        self.objects.push(Box::new(entity));
    }
    pub fn update(&mut self) {
        for id in &self.component_type_ids {
            for o in &self.objects {
                o.update_entity(id);
            }
        }
    }
}
