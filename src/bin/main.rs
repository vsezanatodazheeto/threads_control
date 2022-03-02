use threads_control::ThreadPool;

#[allow(dead_code)]
fn thread_manager<F, T>(data: Vec<T>, f: F) -> Result<Vec<Option<T>>, String>
where
    F: FnOnce(T) -> T,
    F: 'static + Send + Copy,
    T: 'static + Send,
{
    // do nothing if length eq 0
    if data.len() == 0 {
        return Err("Vector is empty!".to_string());
    }

    // vector for the result with length eq to the length of the input vector
    let mut res = Vec::new();
    res.resize_with(data.len(), || None);

    // create pool
    let mut pool = ThreadPool::new(data.len());

    // send tasks
    for (pos, item) in data.into_iter().enumerate() {
        pool.execute(pos, move || f(item));
    }

    // wait for result
    pool.result(&mut res);

    Ok(res)
}

fn main() {
    println!("here we go again");
}

#[cfg(test)]
mod tests {
    use super::*;

    // fn test1(s: String) -> String {
    //     use core::time::Duration;
    //     use std::thread;
    //     println!("{}", s.to_uppercase());
    //     thread::sleep(Duration::from_secs(1));
    //     s.to_uppercase()
    // }

    #[test]
    fn test1() {
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
        ];

        assert_eq!(thread_manager(strings, |s| s.to_uppercase()), Ok(result));
    }

    #[test]
    fn test2() {
        assert_eq!(
            thread_manager(vec![], |x: String| x.to_uppercase()),
            Err("Vector is empty!".to_string())
        );
    }
}
