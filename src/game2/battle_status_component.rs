use game2::entity_component::*;

pub struct BattleStatusComponent {
    hp: i32,
    attack_pow: i32,
}

impl BattleStatusComponent {
    pub fn new() -> BattleStatusComponent {
        BattleStatusComponent {
            hp:100,
            attack_pow:10,
        }
    }
    pub fn hp(&self) -> i32 {
        self.hp
    }
    pub fn attack_pow(&self) -> i32 {
        self.attack_pow
    }
    pub fn attacked(&mut self, by:&BattleStatusComponent) {
        self.hp -= by.attack_pow();
    }
    pub fn update(&mut self, parent:&Entity ) {

    }

}
