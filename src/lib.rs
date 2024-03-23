use std::error::Error;

pub struct ThreadPool;

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

        Ok(ThreadPool)
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
