pub trait IConsumable {
    fn consume(&mut self) -> Option<u32>;
}

pub struct Tuner {
    pub addition_percentage: i32,
    pub addend: i32,
    pub time_left: Option<u32>
}

impl IConsumable for Tuner {
    fn consume(&mut self) -> Option<u32> {
        match &mut self.time_left {
            Some(time_left) => { *time_left -= 1; Some(*time_left) },
            None => None,
        }
    }
}