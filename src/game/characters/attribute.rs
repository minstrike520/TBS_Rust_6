use crate::game::tools;

pub struct Attribute {
    pub base: u32,
    pub addition_percentage: i32,
    pub addend: i32,
}
impl Attribute {
    pub fn new(base: u32) -> Self {
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
    pub fn new(i_atk: u32, i_spd: u32, i_mhp: u32) -> Self {
        let atk = Attribute::new(i_atk);
        let spd = Attribute::new(i_spd);
        let mhp = Attribute::new(i_mhp);
        Self { atk, spd, mhp }
    }
}