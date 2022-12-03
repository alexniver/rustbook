fn main() {}

pub struct AvgCollection {
    list: Vec<i32>,
    avg: f64,
}

impl AvgCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(v) => {
                self.update_avg();
            }
            None => {}
        }
        result
    }

    pub fn avg(&self) -> f64 {
        self.avg
    }

    fn update_avg(&mut self) {
        let sum: i32 = self.list.iter().sum();
        self.avg = sum as f64 / self.list.len() as f64
    }
}
