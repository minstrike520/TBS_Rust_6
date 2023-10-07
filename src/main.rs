use std::collections::HashMap;

use crate::game::tools::tune;

mod game;

// HashMap<&str, Vec<T: Tuner> >

/*
        HashMap::from([
            ("atk", vec![ TunerFlat(-20) ]),
            ("spd", vec![ TunerPerc(30), TunerFlat(5) ])
        ]) 
*/
fn tune_tool_test() {
    assert_eq!(tune(70, 50, 0), 105);
}

fn attribute_set_and_reset_test() {
    let mut c1 = game::chars::Character::new(1, 70, 100, 2000);
    assert_eq!(c1.attributes.spd.get(), 100);
    c1.attributes.spd.set(50, 30);
    assert_eq!(c1.attributes.spd.get(), 180);
    c1.attributes.spd.reset();
    assert_eq!(c1.attributes.spd.get(), 100);
}

fn character_stat_test() {
    let mut c1 = game::chars::Character::new(1, 70, 100, 2000);
    c1.hp.cost(20);
    assert_eq!(c1.hp.get(), 1980);
    c1.hp.add(1000);
    assert_eq!(c1.hp.get(), 2000);
    c1.hp.cost(10000);
    assert_eq!(c1.hp.get(), 0);
}

fn normal_attack_test() {
    let c1 = game::chars::Character::new(1, 70, 100, 2000);
    assert_eq!(c1.attributes.atk.get(), 70);
    let mut c2 = game::chars::Character::new(2, 70, 100, 2000);
    game::skills::normal_attack(&c1, &mut c2);
    assert_eq!(c2.hp.get(), 1965);
}

fn main() {
    normal_attack_test();
    character_stat_test();
    tune_tool_test();
    attribute_set_and_reset_test();
}