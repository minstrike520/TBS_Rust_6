use crate::game::{tools, characters::effects::tuners::IConsumable};
use super::effects::tuners::Tuner;

pub trait IAttribute {
    fn new(base: u32) -> Self;
    fn get(&self) -> u32 ;
    fn add(&mut self, addition_percentage:i32, addend:i32);
    fn set(&mut self, addition_percentage:i32, addend:i32);
    fn reset(&mut self);
}
pub trait ITunable {
    fn add_tuner(&mut self, tuner: Tuner);
    fn is_run_out(&mut self, tuner_index: usize) -> bool;
    fn evaluate(&mut self, tuner_index: usize) -> bool;
    fn evaluate_all(&mut self);
}

pub struct Attribute {
    base: u32,
    addition_percentage: i32,
    addend: i32,
    tuners: Vec<Tuner>,
}
impl IAttribute for Attribute {
    fn new(base: u32) -> Self {
        Self { base, addition_percentage: 0, addend: 0, tuners: Vec::new() }
    }
    fn get(&self) -> u32 {
        tools::tune(self.base, self.addition_percentage, self.addend)
    }
    fn add(&mut self, addition_percentage:i32, addend:i32) {
        self.addition_percentage += addition_percentage;
        self.addend += addend;
    }
    fn set(&mut self, addition_percentage:i32, addend:i32) {
        self.addition_percentage = addition_percentage;
        self.addend = addend;
    }
    fn reset(&mut self) {
        self.addition_percentage = 0;
        self.addend = 0;
    }
}
impl ITunable for Attribute {
    fn add_tuner(&mut self, tuner: Tuner) {
        self.tuners.push(tuner);
    }
    fn is_run_out(&mut self, tuner_index: usize) -> bool {
        if let Some(time_left) = self.tuners[tuner_index].consume() {
            return time_left == 0
        } false
    }
    fn evaluate(&mut self, tuner_index: usize) -> bool {
        let tuner = &self.tuners[tuner_index];
        self.add(tuner.addition_percentage, tuner.addend);
        self.is_run_out(tuner_index)
    }
    fn evaluate_all(&mut self) {
        self.reset();
        let mut removing_indexes: Vec<usize> = Vec::new();
        for tuner_index in 0..self.tuners.len()
            { if self.evaluate(tuner_index) 
                { removing_indexes.push(tuner_index); } }
        for index in removing_indexes { self.tuners.remove(index); }
            
        
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