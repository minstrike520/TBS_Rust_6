use crate::game::{
    chars, tools
};


pub fn normal_attack (caster: &chars::Character, target: &mut chars::Character){
    // target.notice_event(Event::ValueEvent(ValueEvent::Attacked));
    let val: i32 = tools::tune(caster.attributes.atk.get(), -50, 0);
    target.hp -= val;
}
/*
pub fn instant_regen (caster: &chars::Character) {
    let val: i32 = 
}
*/