pub struct ProgressTracker {
    counter: usize,
    min_val: usize,
    max_val: usize,
}

impl ProgressTracker {
    // create object with default values
    pub fn new() -> Self {
        ProgressTracker {
            counter: 0,
            min_val: 0,
            max_val: 100,
        }
    }

    // reset the state to given value
    pub fn init(&mut self, min_val: usize, max_val: usize) {
        self.counter = min_val;
        self.min_val = min_val;
        self.max_val = max_val;
    }

    // update inner state to value
    pub fn update(&mut self, new_v: usize) {
        if new_v >= self.min_val && new_v <= self.max_val {
            self.counter = new_v;
        }
    }

    // returns value in range [0.0, 1.0]
    pub fn get_percentage(&self) -> f32 {
        (self.counter - self.min_val) as f32 / (self.max_val - self.min_val) as f32
    }

    // print the result to terminal
    pub fn show(&self) {
        print!("\r{:2}%", (self.get_percentage() * 100.0).ceil()) //TODO: do erasing
    }
}