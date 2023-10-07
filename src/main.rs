use std::collections::HashMap;

mod game;

// HashMap<&str, Vec<T: Tuner> >

/*
        HashMap::from([
            ("atk", vec![ TunerFlat(-20) ]),
            ("spd", vec![ TunerPerc(30), TunerFlat(5) ])
        ]) 
*/

fn normal_attack_test() {
    let c1 = game::chars::Character::new(1, 70, 100, 2000);
    assert_eq!(c1.attributes.atk.get(), 70);
    let mut c2 = game::chars::Character::new(2, 70, 100, 2000);
    game::skills::normal_attack(&c1, &mut c2);
    assert_eq!(c2.hp, 1965);
}

fn main() {
    normal_attack_test()
    
}