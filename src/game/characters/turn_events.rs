use crate::game::turn_events;
use super::Character;

impl turn_events::ITurnEventObject for Character {
    fn any_ending(&mut self) {
        // self.events.any_ending();
    }
}