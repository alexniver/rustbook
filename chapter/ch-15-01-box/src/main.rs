fn main() {
    let b = Box::new(5);
    println!("b : {}", b);

    use crate::ConsList::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

enum ConsList {
    Cons(i32, Box<ConsList>),
    Nil,
}
