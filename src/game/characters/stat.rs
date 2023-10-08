pub struct CharacterStat {
    max: f32,
    value: f32,
}

impl CharacterStat {
    pub fn new(max: u32) -> Self { Self { max: max as f32, value: max as f32 } }
    pub fn get(&self) -> u32 { self.value.ceil() as u32 }
    pub fn getf(&self) -> f32 { self.value }
    pub fn add(&mut self, val: u32) {
        if self.value + val as f32 > self.max { self.value = self.max; }
        else { self.value += val as f32; }
    }
    pub fn addf(&mut self, val: f32) {
        if self.value + val > self.max { self.value = self.max; }
        else { self.value += val; }
    }
    pub fn cost(&mut self, val: u32) {
        if (self.value - val as f32) < 0.0 { self.value = 0.0; }
        else { self.value -= val as f32; }
    }
    pub fn costf(&mut self, val: f32) {
        if (self.value - val) < 0.0 { self.value = 0.0; }
        else { self.value -= val; }
    }
    pub fn cost_all(&mut self) -> u32 {
        let remain = self.value;
        self.costf(self.value);
        remain.ceil() as u32
    }
    pub fn reset(&mut self) {
        self.value = self.max
    }
    pub fn set_max(&mut self, after: u32) {
        let before = self.max;
        self.max = after as f32;
        let scale = after as f32 / before;
        self.value = (self.value * scale).round();
    }
}
