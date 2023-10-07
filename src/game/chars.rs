use crate::game::tools;
pub struct Attribute {
    pub base: i32,
    pub multiplier_percentage: i32,
    pub addend: i32,
}
impl Attribute {
    fn new(base: i32) -> Self {
        Self { base, multiplier_percentage: 0, addend: 0 }
    }
    pub fn get(&self) -> i32 {
        tools::tune(self.base, self.multiplier_percentage, self.addend)
    }
    pub fn set(&mut self, multiplier_percentage:i32, addend:i32) {
        self.multiplier_percentage = multiplier_percentage;
        self.addend = addend;
    }
    pub fn reset(&mut self) {
        self.multiplier_percentage = 0;
        self.addend = 0;
    }
}
pub struct CharacterAttributes {
    pub atk: Attribute,
    pub spd: Attribute,
    pub mhp: Attribute,
}
impl CharacterAttributes {
    fn new(i_atk: i32, i_spd: i32, i_mhp: i32) -> Self {
        let atk = Attribute::new(i_atk);
        let spd = Attribute::new(i_spd);
        let mhp = Attribute::new(i_mhp);
        Self { atk, spd, mhp }
    }
}

pub struct CharacterStat {
    max: i32,
    value: i32,
}

impl CharacterStat {
    pub fn new(max: i32) -> Self { Self { max, value: max } }
    pub fn get(&self) -> i32 { self.value }
    pub fn add(&mut self, val: i32) {
        if self.value + val > self.max { self.value = self.max; }
        else { self.value += val; }
    }
    pub fn cost(&mut self, val: i32) {
        if self.value - val < 0 { self.value = 0; }
        else { self.value -= val; }
    }
    pub fn set_max(&mut self, after: i32) {
        let before: i32 = self.max;
        self.max = after;
        let scale = after as f32 / before as f32;
        self.value = (self.value as f32 * scale + 0.5).floor() as i32;
    }
}

pub struct Character {
    pub id: i32,
    pub attributes: CharacterAttributes,
    pub hp: CharacterStat,
}

impl Character {
    pub fn new(id: i32, i_atk: i32, i_spd: i32, i_mhp: i32) -> Self {
        Self {
            id,
            attributes: CharacterAttributes::new ( i_atk, i_spd, i_mhp ),
            hp: CharacterStat::new(i_mhp)
        }
    }
}