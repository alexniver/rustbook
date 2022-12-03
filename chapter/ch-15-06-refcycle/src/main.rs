use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

fn main() {
    let child = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("child parent: {:?}", child.parent.borrow().upgrade());
    println!(
        "child: string count:{}, weak count: {}",
        Rc::strong_count(&child),
        Rc::weak_count(&child)
    );
    let parent = Rc::new(Node {
        value: 0,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&child)]),
    });

    *child.parent.borrow_mut() = Rc::downgrade(&parent);

    println!(
        "child: strong count:{}, weak count: {}",
        Rc::strong_count(&child),
        Rc::weak_count(&child)
    );

    println!("child parent: {:?}", child.parent.borrow().upgrade());
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
