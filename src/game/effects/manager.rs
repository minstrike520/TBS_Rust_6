use crate::game::turn_events;
use crate::game::effects;
// use crate::game::characters::Character;

pub trait IEffectManager {
    fn consume();
}

pub struct EffectManager(pub Vec<effects::Effect>);


impl turn_events::ITurnEventObject for EffectManager {
    fn any_ending(&mut self) {
        // 
    }
}