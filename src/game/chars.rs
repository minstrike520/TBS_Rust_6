use crate::game::{tools, constants};
pub struct Attribute {
    pub base: u32,
    pub addition_percentage: i32,
    pub addend: i32,
}
impl Attribute {
    fn new(base: u32) -> Self {
        Self { base, addition_percentage: 0, addend: 0 }
    }
    pub fn get(&self) -> u32 {
        tools::tune(self.base, self.addition_percentage, self.addend)
    }
    pub fn add(&mut self, addition_percentage:i32, addend:i32) {
        self.addition_percentage += addition_percentage;
        self.addend += addend;
    }
    pub fn set(&mut self, addition_percentage:i32, addend:i32) {
        self.addition_percentage = addition_percentage;
        self.addend = addend;
    }
    pub fn reset(&mut self) {
        self.addition_percentage = 0;
        self.addend = 0;
    }
}
pub struct CharacterAttributes {
    pub atk: Attribute,
    pub spd: Attribute,
    pub mhp: Attribute,
}
impl CharacterAttributes {
    fn new(i_atk: u32, i_spd: u32, i_mhp: u32) -> Self {
        let atk = Attribute::new(i_atk);
        let spd = Attribute::new(i_spd);
        let mhp = Attribute::new(i_mhp);
        Self { atk, spd, mhp }
    }
}

pub struct CharacterStat {
    max: f32,
    value: f32,
}

impl CharacterStat {
    pub fn new(max: u32) -> Self { Self { max: max as f32, value: max as f32 } }
    pub fn get(&self) -> u32 { self.value.ceil() as u32 }
    pub fn getf(&self) -> f32 { self.value }
    pub fn add(&mut self, val: u32) {
        if self.value + val as f32 > self.max { self.value = self.max; }
        else { self.value += val as f32; }
    }
    pub fn addf(&mut self, val: f32) {
        if self.value + val > self.max { self.value = self.max; }
        else { self.value += val; }
    }
    pub fn cost(&mut self, val: u32) {
        if (self.value - val as f32) < 0.0 { self.value = 0.0; }
        else { self.value -= val as f32; }
    }
    pub fn costf(&mut self, val: f32) {
        if (self.value - val) < 0.0 { self.value = 0.0; }
        else { self.value -= val; }
    }
    pub fn cost_all(&mut self) -> u32 {
        let remain = self.value;
        self.costf(self.value);
        remain.ceil() as u32
    }
    pub fn reset(&mut self) {
        self.value = self.max
    }
    pub fn set_max(&mut self, after: u32) {
        let before = self.max;
        self.max = after as f32;
        let scale = after as f32 / before;
        self.value = (self.value * scale).round();
    }
}

pub struct Character {
    pub id: i32,
    pub attributes: CharacterAttributes,
    pub hp: CharacterStat,
    pub actv: CharacterStat
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
            attributes: CharacterAttributes::new ( i_atk, i_spd, i_mhp ),
            hp: CharacterStat::new(i_mhp),
            actv: CharacterStat::new(constants::INIT_ACTV),
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