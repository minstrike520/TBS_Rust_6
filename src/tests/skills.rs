use crate::game::{ characters, skills };

pub fn test_all() {
    normal_attack_test();
    instant_regen_test();
}

fn normal_attack_test() {
    let c1 = characters::Character::new(1, 70, 100, 2000);
    assert_eq!(c1.attributes.atk.get(), 70);
    let mut c2 = characters::Character::new(2, 70, 100, 2000);
    skills::normal_attack(&c1, &mut c2);
    assert_eq!(c2.hp.get(), 1965);
}

fn instant_regen_test() {
    let mut c1 = characters::Character::new(1, 70, 100, 2000);
    c1.hp.cost(1000);
    skills::instant_regen(&mut c1);
    assert_eq!(c1.hp.get(), 1600);
}