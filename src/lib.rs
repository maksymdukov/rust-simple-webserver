use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn execute<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(func)).unwrap();
    }
    /// Create a new Threapool
    ///
    /// The size is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The 'new' function will panic of the size is zero
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            println!("Creating thread");
            let worker = Worker::new(id, Arc::clone(&receiver));
            workers.push(worker);
        }
        Self { workers, sender }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Start job: {}", id);

            job();
            println!("Finished job: {}", id)

            // or
            // while let Ok(rec) = receiver.lock() {
            //     let job = rec.recv().unwrap();
            //     drop(rec);
            //     println!("Worker {} got a job.", id);

            //     job();
            // }
        });
        Self { id, thread }
    }
}
