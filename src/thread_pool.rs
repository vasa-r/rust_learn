use std::thread;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let handle = thread::spawn(move || loop {
            println!("Worker {id} waiting for job...");
        });
        Worker { id, thread: handle }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id));
        }
        ThreadPool { workers }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub fn thread_pool_test() {
    let a = 10;
    let b = a;
    println!("{}, {}", a, b);
}
