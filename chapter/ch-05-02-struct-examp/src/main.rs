fn main() {
    let rect = Rect::new(33.2, 102.3);
    println!("rect : {:?}, area: {}", rect, rect.area());
}

#[derive(Debug)]
struct Rect {
    width: f32,
    height: f32,
}

impl Rect {
    fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> f32 {
        self.width * self.height
    }
}
