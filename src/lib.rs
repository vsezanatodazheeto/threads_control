use std::sync::{mpsc, mpsc::Receiver, mpsc::Sender, Arc, Mutex};
use std::thread;

const NTHREADS: usize = 5;

enum Message<T> {
    Work { pos: usize, job: Job<T> },
    Result { pos: usize, res: T },
    Terminate,
}

pub struct ThreadPool<T> {
    workers: Vec<Worker>,
    sender: Sender<Message<T>>,
    receiver: Receiver<Message<T>>,
}

type Job<T> = Box<dyn FnOnce() -> T + Send + 'static>;

impl<T> ThreadPool<T>
where
    T: 'static + Send,
{
    // Creates thread pool with threads limited by NTHREADS.
    pub fn new(desired_size: usize) -> ThreadPool<T> {
        let size = Self::threads_determine_qt(desired_size);

        let mut workers = Vec::with_capacity(size);

        let (tx1, rx1) = mpsc::channel::<Message<T>>();
        let many_senders = Arc::new(Mutex::new(tx1));

        let (tx2, rx2) = mpsc::channel::<Message<T>>();
        let many_receivers = Arc::new(Mutex::new(rx2));

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
            sender: tx2,
            receiver: rx1,
        }
    }

    // Sends messages of type Job<T> to threads.
    pub fn execute<F>(&self, pos: usize, f: F)
    where
        F: FnOnce() -> T,
        F: 'static + Send,
        T: 'static + Send,
    {
        let job = Box::new(f);
        self.sender.send(Message::Work { pos, job }).unwrap();
    }

    // Receives result of Job<T> in threads. It stop works when all threads closes
    // their 'senders' of channel.
    pub fn result(&mut self, result: &mut Vec<Option<T>>) {
        self.terminate();
        loop {
            let recv_result = self.receiver.recv();
            match recv_result {
                Ok(Message::Result { pos, res }) => {
                    result[pos] = Some(res);
                }
                _ => {
                    break;
                }
            }
        }
    }

    // Determines the number of threads.
    fn threads_determine_qt(desired_size: usize) -> usize {
        assert!(desired_size > 0);
        if desired_size < NTHREADS {
            desired_size
        } else {
            NTHREADS
        }
    }

    // Sends 'Terminate' message to stop working into threads.
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
    thread: Option<thread::JoinHandle<()>>,
}

type WorkSender<T> = Arc<Mutex<Sender<Message<T>>>>;
type WorkReceiver<T> = Arc<Mutex<Receiver<Message<T>>>>;

impl Worker {
    // Receives from ThreadPool Messages of type Work or Terminate.
    // In case of Work type, do job and send result of job.
    // In case of Termiante type , drops 'sender' side of channel and breaks from loop.
    fn new<T>(_id: usize, receiver: WorkReceiver<T>, sender: WorkSender<T>) -> Worker
    where
        T: 'static + Send,
    {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::Work { pos, job } => {
                    // println!("Worker {} got a job", _id);
                    let res = job();
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
            thread: Some(thread),
        }
    }
}
