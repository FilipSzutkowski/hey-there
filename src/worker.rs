use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

pub struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;

type JobReceiver = mpsc::Receiver<Job>;

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<JobReceiver>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker '{id}': got a job, executing.");

            job();
        });

        Worker { id, thread }
    }
}
