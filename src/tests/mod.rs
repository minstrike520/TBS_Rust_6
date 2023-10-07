pub mod chars;
pub mod skills;
pub mod tools;
pub mod queue;

pub fn test_all() {
    chars::test_all();
    skills::test_all();
    tools::test_all();
    queue::test_all();
}