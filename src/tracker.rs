use std::cmp::min;

pub struct ProgressTracker {
    counter: usize,
    min_val: usize,
    max_val: usize,
}

/// Note: non-templated version, uses usize which can overflow and this is not checked
impl ProgressTracker {
    // create object with default values
    pub fn new(min_val: usize, max_val: usize) -> Self {
        ProgressTracker {
            counter: min_val,
            min_val,
            max_val,
        }
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

    // increase stored value by 1
    pub fn increment(&mut self) {
        if self.counter < self.max_val {
            self.counter += 1;
        }
    }

    // decrease stored value by 1
    pub fn decrement(&mut self) {
        if self.counter > self.min_val {
            self.counter -= 1;
        }
    }

    pub fn is_done(&self) -> bool {
        self.max_val == self.counter
    }
}