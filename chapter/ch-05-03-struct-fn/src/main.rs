fn main() {
    let rect = Rect::new(323.3, 44.3);
    println!("rect area: {}, is_zero: {}", rect.area(), rect.is_zero());
    let rect = Rect::new(0., 44.3);
    println!("rect area: {}, is_zero: {}", rect.area(), rect.is_zero());
    let rect = Rect::square(33.);
    println!("rect area: {}, is_zero: {}", rect.area(), rect.is_zero());
}

struct Rect {
    width: f32,
    height: f32,
}

impl Rect {
    fn new(width: f32, height: f32) -> Self {
        Rect { width, height }
    }

    fn square(width: f32) -> Self {
        Self {
            width,
            height: width,
        }
    }

    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn is_zero(&self) -> bool {
        self.width == 0. || self.height == 0.
    }
}
