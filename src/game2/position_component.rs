use game2::entity_component::*;

pub struct PositionComponent {
    pos: (f32,f32),
}

impl PositionComponent {
    pub fn new() -> PositionComponent {
        PositionComponent{ pos:(0.0,0.0) }
    }
    pub fn pos(&self) -> (f32,f32) {
        self.pos
    }
    pub fn set_pos(&mut self, pos:(f32,f32)) {
        self.pos = pos;
    }
    pub fn update(&mut self, parent:&Entity) {
        // println!("position!!");
        // if let Component::Input(ref input) = *parent.component(1) {
        //     let tgtpos = input.clicked_pos();
        //     if (tgtpos.0 - self.pos.0).abs() < 5.0 && (tgtpos.1 - self.pos.1).abs() < 5.0 {
        //         return;
        //     }
        //     self.pos.0 = self.pos.0 + (tgtpos.0 - self.pos.0) * 0.05;
        //     self.pos.1 = self.pos.1 + (tgtpos.1 - self.pos.1) * 0.05;
        // }
    }
}
