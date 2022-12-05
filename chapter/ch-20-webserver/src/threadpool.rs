use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: u8) -> ThreadPool {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(size as usize);
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&rx)));
        }

        ThreadPool {
            workers,
            tx: Some(tx),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.tx.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down thread pool...");
        drop(self.tx.take());
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
                println!("shutdown worker {}", worker.id);
            }
        }
        println!("Shutting down thread pool finish!");
    }
}

struct Worker {
    id: u8,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: u8, rx: Arc<Mutex<Receiver<Job>>>) -> Self {
        Self {
            id,
            thread: Some(thread::spawn(move || loop {
                let job = rx.lock().unwrap().recv();
                match job {
                    Ok(job) => {
                        println!("worker id : {id} got a job");
                        job();
                    }
                    Err(_) => {
                        // println!("worker id : {id} disconnected, shutdown");
                        break;
                    }
                }
            })),
        }
    }
}
