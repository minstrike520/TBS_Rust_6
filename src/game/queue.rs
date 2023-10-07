use crate::game::chars;

pub struct CharacterQueue(pub Vec<chars::Character>);

impl CharacterQueue {
    pub fn new() -> Self { Self(Vec::new()) }

    pub fn sort_by_id(&mut self) {
        self.0.sort_by(|a,b| a.id.cmp(&b.id))
    }
    pub fn sort_by_left_steps(&mut self) {
        self.0.sort_by(|a,b| a.getf_left_steps().partial_cmp(&b.getf_left_steps()).unwrap());
    }
    pub fn proceed(&mut self) {
        self.sort_by_left_steps();
        let remain_steps = self.0[0].step_to_front();
        let mut character_in_charge = self.0.remove(0);
        for character in &mut self.0 {
            character.stepf(remain_steps);
        }
        character_in_charge.actv.reset();
        self.0.push(character_in_charge);
    }
}