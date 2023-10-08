use super::Character;
use super::attribute::IAttribute;

pub trait HasActionValue {
    fn getf_left_steps(&self) -> f32;
    fn get_left_steps(&self) -> u32;
    fn stepf(&mut self, steps: f32);
    fn step(&mut self, steps: u32);
    fn step_to_front(&mut self) -> f32;
}

impl HasActionValue for Character {
    fn getf_left_steps(&self) -> f32 {
        self.actv.getf() / self.attributes.spd.get() as f32
    }
    fn get_left_steps(&self) -> u32 {
        self.getf_left_steps().ceil() as u32
    }
    fn stepf(&mut self, steps: f32) {
        self.actv.costf( self.attributes.spd.get() as f32 * steps);
    }
    fn step(&mut self, steps: u32) { self.stepf(steps as f32) }
    fn step_to_front(&mut self) -> f32 {
        let remain_steps = self.getf_left_steps();
        self.actv.cost_all();
        remain_steps
    }
}
