use crate::game::tools;

pub fn test_all() {
    tune_tool_test();
}

fn tune_tool_test() {
    assert_eq!(tools::tune(70, 50, 0), 105);
}