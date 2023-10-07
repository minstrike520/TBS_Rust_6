use crate::game::chars;

pub struct CharacterQueue(pub Vec<chars::Character>);

impl CharacterQueue {
    pub fn new() -> Self { Self(Vec::new()) }

    pub fn sort_by_id(&mut self) {
        self.0.sort_by(|a,b| a.id.cmp(&b.id))
    }
}