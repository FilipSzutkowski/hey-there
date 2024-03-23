mod worker;

use std::{
    error::Error,
    sync::{mpsc, Arc, Mutex},
};
use worker::{Job, Worker};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Errors
    ///
    /// The `new` function will return an `Error` if the size is zero.
    pub fn new(size: usize) -> Result<ThreadPool, Box<dyn Error>> {
        if size < 1 {
            return Err("Specified thread number can't be less than one.".into());
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take().unwrap());

        for worker in &mut self.workers {
            println!("Worker {}: shutting down", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            } else {
                eprintln!("Worker '{}': Error: Could not join thread.", worker.id);
            }
        }
    }
}
