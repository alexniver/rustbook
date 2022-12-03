use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handle = vec![];

    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        handle.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }))
    }

    for h in handle {
        h.join().unwrap();
    }

    println!("counter : {:?}", counter.lock().unwrap());
}
