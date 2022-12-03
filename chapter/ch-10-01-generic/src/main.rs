fn main() {
    let integer = Point { x: 1, y: 2 };
    let float = Point { x: 1., y: 2. };

    println!("x : {:?}", integer.x());
    // println!("integer distance: {}", integer.distance_from_origin());
    println!("float distance: {}", float.distance_from_origin());

    let p2_1 = Point2 { x: 1, y: 2. };
    let p2_2 = Point2 { x: 'c', y: "d" };

    let p2_3 = p2_1.mixup(p2_2);
    println!("{:?}", p2_3);
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<R, S>(self, p2: Point2<R, S>) -> Point2<T, S> {
        Point2 { x: self.x, y: p2.y }
    }
}
