pub struct Attribute {
    pub base: i32,
    multiplier_percentage: i32,
    addend: i32,
}
impl Attribute {
    fn new(base: i32) -> Self {
        Self { base, multiplier_percentage: 0, addend: 0 }
    }
    pub fn get(&self) -> i32 {
        (self.base as f32 * (1.0 + self.multiplier_percentage as f32 / 100.0) + self.addend as f32 + 0.5).floor() as i32
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

pub struct Character {
    pub id: i32,
    pub attributes: CharacterAttributes,
    pub hp: i32,
}

impl Character {
    pub fn new(id: i32, i_atk: i32, i_spd: i32, i_mhp: i32) -> Self {
        Self {
            id,
            attributes: CharacterAttributes::new ( i_atk, i_spd, i_mhp ),
            hp: i_mhp
        }
    }
}