use crate::game::{ chars, skills };

pub fn test_all() {
    normal_attack_test();
}

fn normal_attack_test() {
    let c1 = chars::Character::new(1, 70, 100, 2000);
    assert_eq!(c1.attributes.atk.get(), 70);
    let mut c2 = chars::Character::new(2, 70, 100, 2000);
    skills::normal_attack(&c1, &mut c2);
    assert_eq!(c2.hp.get(), 1965);
}