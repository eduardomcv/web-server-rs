use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[derive(Debug)]
pub struct PoolCreationError;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("Worker {id} failed to aquire lock")
                .recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
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

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let worker = Worker::new(id, Arc::clone(&receiver));
            workers.push(worker);
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Creates a new ThreadPool
    ///
    /// The size is the number of threads in the pool
    ///
    /// This method is similar to ThreadPool::new() but it won't panic.
    /// Instead, this method returns Result<ThreadPool, PoolCreationError>
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size < 1 {
            return Err(PoolCreationError);
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let worker = Worker::new(id, Arc::clone(&receiver));
            workers.push(worker);
        }

        let pool = ThreadPool {
            workers,
            sender: Some(sender),
        };

        Ok(pool)
    }

    /// Sends the provided closure to one of the workers in the ThreadPool
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
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_pool_new() {
        let pool = ThreadPool::new(1);
        assert_eq!(pool.workers.len(), 1);
    }

    #[test]
    fn test_thread_pool_build() -> Result<(), PoolCreationError> {
        let pool = ThreadPool::build(1);
        assert_eq!(pool?.workers.len(), 1);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_thread_pool_new_panic() {
        ThreadPool::new(0);
    }

    #[test]
    #[should_panic]
    fn test_thread_pool_build_error() {
        ThreadPool::build(0).unwrap();
    }

    #[test]
    fn test_thread_pool_execute() {
        let pool = ThreadPool::new(1);
        pool.execute(|| assert!(true));
    }

    #[test]
    fn test_worker() {
        let (sender, receiver) = mpsc::channel();

        let worker = Worker::new(0, Arc::new(Mutex::new(receiver)));
        assert_eq!(worker.id, 0);

        let message = Box::new(|| assert!(true));
        sender.send(message).unwrap();
    }
}
