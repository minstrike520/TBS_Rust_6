use crate::game::chars;

pub fn normal_attack (caster: &chars::Character, target: &mut chars::Character){
    // target.notice_event(Event::ValueEvent(ValueEvent::Attacked));
    let val: i32 = (caster.attributes.atk.get() as f32 * 0.5 + 0.5).floor() as i32;
    target.hp -= val;
}