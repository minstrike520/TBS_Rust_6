use crate::game::characters;

pub fn test_all() {
    attribute_set_and_reset_test();
    character_stat_test();
    character_stat_set_max_test();
    character_tune_and_reset_mhp_test();
    character_steps_test();

}

fn attribute_set_and_reset_test() {
    let mut spd = characters::attribute::Attribute::new(100);
    assert_eq!(spd.get(), 100);
    spd.set(50, 30);
    assert_eq!(spd.get(), 180);
    spd.reset();
    assert_eq!(spd.get(), 100);
}

fn character_stat_test() {
    let mut hp = characters::stat::CharacterStat::new(2000);
    hp.cost(20);
    assert_eq!(hp.get(), 1980);
    hp.add(1000);
    assert_eq!(hp.get(), 2000);
    hp.cost(10000);
    assert_eq!(hp.get(), 0);
}

fn character_stat_set_max_test() {
    let mut hp = characters::stat::CharacterStat::new(2000);
    hp.set_max(2400);
    assert_eq!(hp.get(), 2400);
    hp.cost(400);
    assert_eq!(hp.get(), 2000);
    hp.set_max(1200);
    assert_eq!(hp.get(), 1000);
}

fn character_tune_and_reset_mhp_test() {
    let mut c1 = characters::Character::new(1, 70, 100, 2000);
    c1.hp.cost(200);
    c1.tune_mhp(20, 0);
    assert_eq!(c1.attributes.mhp.get(), 2400);
    assert_eq!(c1.hp.get(), 2160);
    c1.reset_mhp();
    assert_eq!(c1.hp.get(), 1800);
}

fn character_steps_test() {
    use characters::action_value::HasActionValue;
    let mut c1 = characters::Character::new(1, 70, 100, 2000);
    assert_eq!(c1.get_left_steps(), 100);
    c1.step(50);
    assert_eq!(c1.get_left_steps(), 50);
    assert_eq!(c1.actv.get(), 5000);
    c1.step_to_front();
    assert_eq!(c1.get_left_steps(), 0);
    c1.actv.reset();
    assert_eq!(c1.get_left_steps(), 100);
}