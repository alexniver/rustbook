use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

fn main() {
    let m = MyBox::new("file.txt");
    let m2 = MyBox::new("file2.txt");
    drop(m);
    drop(m2);
}

struct MyBox<T: Display + Debug> {
    data: T,
}

impl<T: Display + Debug> MyBox<T> {
    fn new(t: T) -> MyBox<T> {
        MyBox { data: t }
    }
}

impl<T: Display + Debug> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Display + Debug> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("dropping {:?}", self.data);
    }
}
