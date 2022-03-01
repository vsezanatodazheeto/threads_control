use std::sync::{mpsc, mpsc::Receiver, mpsc::Sender, Arc, Mutex};
use std::thread;

const NTHREADS: usize = 5;

enum Message<T> {
    Work { pos: usize, job: Job<T> },
    Result { pos: usize, res: T },
    Terminate,
}

struct ThreadPool<T> {
    workers: Vec<Worker>,
    sender: Sender<Message<T>>,
    receiver: Receiver<Message<T>>,
}

impl<T: 'static + Send + std::fmt::Debug> ThreadPool<T> {
    pub fn new(size: usize) -> ThreadPool<T> {
        let size = Self::threads_determine_qt(size);
        let mut workers = Vec::with_capacity(size);

        let pipe1: (Sender<Message<T>>, Receiver<Message<T>>) = mpsc::channel();
        let many_senders = Arc::new(Mutex::new(pipe1.0));

        let pipe2: (Sender<Message<T>>, Receiver<Message<T>>) = mpsc::channel();
        let many_receivers = Arc::new(Mutex::new(pipe2.1));

        for worker_id in 0..size {
            workers.push(Worker::new(
                worker_id,
                many_receivers.clone(),
                many_senders.clone(),
            ));
        }
        drop(many_senders);
        drop(many_receivers);
        ThreadPool {
            workers,
            sender: pipe2.0,
            receiver: pipe1.1,
        }
    }

    pub fn execute<F>(&self, pos: usize, f: F)
    where
        F: FnOnce() -> T,
        F: 'static + Send,
        T: 'static + Send,
    {
        let job = Box::new(f);
        self.sender.send(Message::Work { pos, job }).unwrap();
    }

    pub fn result(&self, vec_r: &mut Vec<Option<T>>) {
        self.terminate();
        loop {
            let recv_result = self.receiver.recv();
            match recv_result {
                Ok(Message::Result { pos, res }) => {
                    vec_r[pos] = Some(res);
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn threads_determine_qt(desired_size: usize) -> usize {
        assert!(desired_size > 0);
        if desired_size < NTHREADS {
            desired_size
        } else {
            NTHREADS
        }
    }

    fn terminate(&self) {
        for _ in 0..self.workers.len() {
            self.sender.send(Message::Terminate).unwrap();
        }
    }
}

impl<T> Drop for ThreadPool<T> {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new<T: 'static + Send + std::fmt::Debug>(
        id: usize,
        receiver: Arc<Mutex<Receiver<Message<T>>>>,
        sender: Arc<Mutex<Sender<Message<T>>>>,
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::Work { pos, job } => {
                    println!("Worker {} got a job", id);
                    let res = job();
                    println!("res: {:?}", res);
                    sender
                        .lock()
                        .unwrap()
                        .send(Message::Result { pos, res })
                        .unwrap();
                }
                _ => {
                    drop(sender);
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

type Job<T> = Box<dyn FnOnce() -> T + Send + 'static>;

pub fn thread_manager<F, T>(data: Vec<T>, f: F)
where
    F: FnOnce(T) -> T,
    F: 'static + Send + Copy,
    T: 'static + Send,
{
    let mut result = Vec::new();
    result.resize_with(data.len(), || None);

    let pool = ThreadPool::new(data.len());

    // send tasks
    for (pos, item) in data.into_iter().enumerate() {
        pool.execute(pos, move || {
            f(item);
        });
    }

    pool.result(&mut result);

    println!("{:?}", result);
}
