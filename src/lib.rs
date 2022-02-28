// use std::fmt::Debug;
use std::sync::{mpsc, mpsc::Receiver, mpsc::Sender, Arc, Mutex};
use std::thread;

const NTHREADS: usize = 5;

enum Message<T> {
    Work { pos: usize, job: Job<T> },
    Result { pos: usize, res: T },
    Terminate,
}

type Job<T> = Box<dyn FnOnce() -> T + Send + 'static>;

pub fn thread_manager<F, T>(data: Vec<T>, f: F) -> Vec<Option<T>>
where
    F: FnOnce(T) -> T + Send + Copy + 'static,
    T: Send + 'static,
{
    if data.len() == 0 {
        return Vec::new();
    }

    let nthreads = if data.len() < NTHREADS {
        data.len()
    } else {
        NTHREADS
    };

    let mut result = Vec::new();
    result.resize_with(data.len(), || None);

    let mut thread_handle = Vec::with_capacity(NTHREADS);

    let pipe1: (Sender<Message<T>>, Receiver<Message<T>>) = mpsc::channel();
    let many_senders = Arc::new(Mutex::new(pipe1.0));

    let pipe2: (Sender<Message<T>>, Receiver<Message<T>>) = mpsc::channel();
    let many_receivers = Arc::new(Mutex::new(pipe2.1));

    // // wait for tasks, run tasks, send result
    for _worker in 0..nthreads {
        let sender = many_senders.clone();
        let receiver = many_receivers.clone();
        thread_handle.push(thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::Work { pos, job } => {
                    // println!("Worker {} got a job", _worker);
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
        }));
    }

    // send tasks
    for (pos, item) in data.into_iter().enumerate() {
        let job = Box::new(move || f(item));
        pipe2.0.send(Message::Work { pos, job }).unwrap();
    }

    // send terminate
    for _ in 0..NTHREADS {
        pipe2.0.send(Message::Terminate).unwrap();
    }

    // wait result
    drop(many_senders);
    loop {
        let recv_result = pipe1.1.recv();
        match recv_result {
            Ok(Message::Result { pos, res }) => {
                result[pos] = Some(res);
                // result.push(res);
            }
            _ => {
                break;
            }
        }
    }

    // handling before return
    for thread in thread_handle {
        thread.join().unwrap();
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::thread_manager;
    use std::thread;
    use std::time::Duration;

    fn test1(s: String) -> String {
        println!("{}", s.to_uppercase());
        thread::sleep(Duration::from_secs(1));
        s.to_uppercase()
    }

    #[test]
    fn ex1() {
        let strings = vec![
            "test 1".to_string(),
            "test 2".to_string(),
            "test 3".to_string(),
            "test 4".to_string(),
            "test 5".to_string(),
            "test 6".to_string(),
            "test 7".to_string(),
            "test 8".to_string(),
            "test 9".to_string(),
            "test 10".to_string(),
            "test 11".to_string(),
            "test 12".to_string(),
            "test 13".to_string(),
            "test 14".to_string(),
            "test 15".to_string(),
            "test 16".to_string(),
            "test 17".to_string(),
            "test 18".to_string(),
        ];
        let result = vec![
            Some("TEST 1".to_string()),
            Some("TEST 2".to_string()),
            Some("TEST 3".to_string()),
            Some("TEST 4".to_string()),
            Some("TEST 5".to_string()),
            Some("TEST 6".to_string()),
            Some("TEST 7".to_string()),
            Some("TEST 8".to_string()),
            Some("TEST 9".to_string()),
            Some("TEST 10".to_string()),
            Some("TEST 11".to_string()),
            Some("TEST 12".to_string()),
            Some("TEST 13".to_string()),
            Some("TEST 14".to_string()),
            Some("TEST 15".to_string()),
            Some("TEST 16".to_string()),
            Some("TEST 17".to_string()),
            Some("TEST 18".to_string()),
        ];

        assert_eq!(thread_manager(strings, test1), result);
    }

    #[test]
    fn ex2() {
        let strings = vec![
            "test 1".to_string(),
            "test 2".to_string(),
            "test 3".to_string(),
        ];
        let result = vec![
            Some("TEST 1".to_string()),
            Some("TEST 2".to_string()),
            Some("TEST 3".to_string()),
        ];

        assert_eq!(thread_manager(strings, test1), result);
    }
}
