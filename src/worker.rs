use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

pub struct Worker {
    pub id: usize,
    pub thread: Option<JoinHandle<()>>,
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;

type JobReceiver = mpsc::Receiver<Job>;

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<JobReceiver>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();

            match job {
                Ok(job) => {
                    println!("Worker '{id}': got a job, executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker '{id}': Shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
