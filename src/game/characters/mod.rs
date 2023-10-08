pub mod attribute;
pub mod stat;

use crate::game::constants;

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
    pub fn getf_left_steps(&self) -> f32 {
        self.actv.getf() / self.attributes.spd.get() as f32
    }
    pub fn get_left_steps(&self) -> u32 {
        self.getf_left_steps().ceil() as u32
    }
    pub fn stepf(&mut self, steps: f32) {
        self.actv.costf( self.attributes.spd.get() as f32 * steps);
    }
    pub fn step(&mut self, steps: u32) { self.stepf(steps as f32) }
    pub fn step_to_front(&mut self) -> f32 {
        let remain_steps = self.getf_left_steps();
        self.actv.cost_all();
        remain_steps
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