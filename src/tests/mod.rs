pub mod chars;
pub mod skills;
pub mod tools;

pub fn test_all() {
    chars::test_all();
    skills::test_all();
    tools::test_all();
}