use core::time::Duration;
use std::thread;
// use std::fmt::Debug;

use threads_control::ThreadPool;

fn test1(s: String) -> String {
    println!("{}", s.to_uppercase());
    thread::sleep(Duration::from_secs(1));
    s.to_uppercase()
}

fn thread_manager<F, T>(data: Vec<T>, f: F) -> Vec<Option<T>>
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
        pool.execute(pos, move || f(item));
    }

    // wait for result
    pool.result(&mut result);

	result
}

fn main() {
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
        "test 19".to_string(),
        "test 20".to_string(),
    ];

    let res = thread_manager(strings, test1);
	println!("{:?}", res);
}
