use std::{thread, time::Duration};

fn main() {
    let mut vec = vec![];

    let handle = thread::spawn(move || {
        for i in 0..10 {
            vec.push(i);
        }
        println!("vec size : {}", vec.len());
    });

    handle.join().unwrap();
}
