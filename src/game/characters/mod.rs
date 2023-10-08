pub mod attribute;
pub mod stat;
pub mod action_value;
pub mod queue;
pub mod effects;

use crate::game::constants;
use attribute::IAttribute;

pub struct Character {
    pub id: i32,
    pub attributes: attribute::CharacterAttributes,
    pub hp: stat::CharacterStat,
    pub actv: stat::CharacterStat
}

impl std::fmt::Debug for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Character")
        .field("id", &self.id)
        .finish()
    }
}
impl PartialEq for Character {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Character {
    pub fn new(id: i32, i_atk: u32, i_spd: u32, i_mhp: u32) -> Self {
        Self {
            id,
            attributes: attribute::CharacterAttributes::new ( i_atk, i_spd, i_mhp ),
            hp: stat::CharacterStat::new(i_mhp),
            actv: stat::CharacterStat::new(constants::INIT_ACTV),
                    }
    }
    pub fn mock_by_id(id: i32) -> Self {
        Self::new(id, 0, 0, 0)
    }
    pub fn tune_mhp(&mut self, addition_percentage: i32, addend: i32) {
        self.attributes.mhp.set(addition_percentage, addend);
        self.hp.set_max(self.attributes.mhp.get());
    }
    pub fn reset_mhp(&mut self) {
        self.attributes.mhp.reset();
        self.hp.set_max(self.attributes.mhp.get());
    }
}