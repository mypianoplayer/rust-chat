extern crate rand;

use game2::entity_component::*;

pub struct BattleStatusComponent {
    hp: i32,
    attack_pow: i32,
}

impl BattleStatusComponent {
    pub fn new() -> BattleStatusComponent {
        BattleStatusComponent {
            hp:100,
            attack_pow:15,
        }
    }
    pub fn hp(&self) -> i32 {
        self.hp
    }
    pub fn attack_pow(&self) -> i32 {
        self.attack_pow
    }
    pub fn attacked(&mut self, by:&BattleStatusComponent) -> i32 {
        let damage = by.attack_pow() + (rand::random::<u32>() % 4_u32) as i32;
        self.hp -= damage;
        // if self.hp < 0 {
        //     self.hp = 0;
        // }
        damage
    }
    pub fn update(&mut self, parent:&Entity ) {

    }

}
