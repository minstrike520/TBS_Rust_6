use crate::game::tools;
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
    max: u32,
    value: u32,
}

impl CharacterStat {
    pub fn new(max: u32) -> Self { Self { max, value: max } }
    pub fn get(&self) -> u32 { self.value }
    pub fn add(&mut self, val: u32) {
        if self.value + val > self.max { self.value = self.max; }
        else { self.value += val; }
    }
    pub fn cost(&mut self, val: u32) {
        if (self.value as i32 - val as i32) < 0 { self.value = 0; }
        else { self.value -= val; }
    }
    pub fn set_max(&mut self, after: u32) {
        let before: u32 = self.max;
        self.max = after;
        let scale = after as f32 / before as f32;
        self.value = (self.value as f32 * scale + 0.5).floor() as u32;
    }
}

pub struct Character {
    pub id: i32,
    pub attributes: CharacterAttributes,
    pub hp: CharacterStat,
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
            hp: CharacterStat::new(i_mhp)
        }
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