use crate::game::characters;
use characters::queue;

pub fn test_all() {
    queue_sort_by_id_test();
    queue_character_partial_eq_test();
    queue_sort_by_left_steps_test();
    queue_proceed_test();
}

fn queue_sort_by_id_test() {
    let mut q1 = queue::CharacterQueue( vec![
        characters::Character::new(1, 10, 10, 100),
        characters::Character::new(3, 10, 10, 100),
        characters::Character::new(2, 10, 10, 100),
    ]);
    q1.sort_by_id();
    assert_eq!(q1.0, vec![
        characters::Character::new(1, 10, 10, 100),
        characters::Character::new(2, 10, 10, 100),
        characters::Character::new(3, 10, 10, 100),
    ])
}

fn queue_character_partial_eq_test() {
    let q1 = queue::CharacterQueue( vec![
        characters::Character::new(1, 154, 10, 100),
        characters::Character::new(3, 1024, 50, 4442),
        characters::Character::new(2, 25, 10, 1207),
    ]);
    assert_eq!(q1.0, vec![
        characters::Character::new(1, 10, 10, 52),
        characters::Character::new(3, 150, 990, 205),
        characters::Character::new(2, 10, 10, 86),
    ])
}

fn queue_sort_by_left_steps_test() {
    let mut q1 = queue::CharacterQueue( vec![
        characters::Character::new(5, 10, 20, 100),
        characters::Character::new(6, 10, 50, 100),
        characters::Character::new(2, 10, 15, 100),
    ]);
    q1.sort_by_left_steps();
    assert_eq!(q1.0, vec![
        characters::Character::new(6, 10, 10, 100),
        characters::Character::new(5, 10, 10, 100),
        characters::Character::new(2, 10, 10, 100),
    ])
}

fn queue_proceed_test() {
    let mut q1 = queue::CharacterQueue( vec![
        characters::Character::new(5, 10, 20, 100),
        characters::Character::new(6, 10, 21, 100),
        characters::Character::new(2, 10, 19, 100),
    ]);
    q1.proceed();
    assert_eq!(q1.0[0], characters::Character::mock_by_id(5));
    q1.proceed();
    assert_eq!(q1.0[0], characters::Character::mock_by_id(2));
    q1.proceed();
    assert_eq!(q1.0[0], characters::Character::mock_by_id(6))
}