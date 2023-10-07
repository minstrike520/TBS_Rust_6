use crate::game::{ queue, chars };

pub fn test_all() {
    queue_sort_by_id_test();
    queue_character_partial_eq_test();
}

fn queue_sort_by_id_test() {
    let mut q1 = queue::CharacterQueue( vec![
        chars::Character::new(1, 10, 10, 100),
        chars::Character::new(3, 10, 10, 100),
        chars::Character::new(2, 10, 10, 100),
    ]);
    q1.sort_by_id();
    assert_eq!(q1.0, vec![
        chars::Character::new(1, 10, 10, 100),
        chars::Character::new(2, 10, 10, 100),
        chars::Character::new(3, 10, 10, 100),
    ])
}

fn queue_character_partial_eq_test() {
    let q1 = queue::CharacterQueue( vec![
        chars::Character::new(1, 154, 10, 100),
        chars::Character::new(3, 1024, 50, 4442),
        chars::Character::new(2, 25, 10, 1207),
    ]);
    assert_eq!(q1.0, vec![
        chars::Character::new(1, 10, 10, 52),
        chars::Character::new(3, 150, 990, 205),
        chars::Character::new(2, 10, 10, 86),
    ])
}