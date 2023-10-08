use crate::game::{
    characters, tools
};


pub fn normal_attack (caster: &characters::Character, target: &mut characters::Character){
    // target.notice_event(Event::ValueEvent(ValueEvent::Attacked));
    let val: u32 = tools::tune(caster.attributes.atk.get(), -50, 0);
    target.hp.cost(val);
}

pub fn instant_regen (caster: &mut characters::Character) {
    let val: u32 = tools::tune(caster.attributes.mhp.get(), -70, 0);
    caster.hp.add(val);
}