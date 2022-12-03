use std::rc::Rc;

use crate::List::*;
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    {
        let b = Cons(6, Rc::clone(&a));
        println!("count after b = {}", Rc::strong_count(&a));
    }
    println!("count after out b = {}", Rc::strong_count(&a));
    let c = Cons(7, Rc::clone(&a));
    println!("count after c = {}", Rc::strong_count(&a));
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
