use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vec = vec!["a", "b", "c", "d"];
        for v in vec.into_iter() {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vec = vec!["A", "B", "C", "D"];
        for v in vec.into_iter() {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for v in rx {
        println!("receive: {}", v);
    }
}
