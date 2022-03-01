use core::time::Duration;
use std::thread;
use threads_control::thread_manager;

fn test1(s: String) -> String {
    println!("{}", s.to_uppercase());
    // thread::sleep(Duration::from_secs(1));
    s.to_uppercase()
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

    // let result = vec![
    //     Some("TEST 1".to_string()),
    //     Some("TEST 2".to_string()),
    //     Some("TEST 3".to_string()),
    //     Some("TEST 4".to_string()),
    //     Some("TEST 5".to_string()),
    //     Some("TEST 6".to_string()),
    //     Some("TEST 7".to_string()),
    //     Some("TEST 8".to_string()),
    //     Some("TEST 9".to_string()),
    //     Some("TEST 10".to_string()),
    //     Some("TEST 11".to_string()),
    //     Some("TEST 12".to_string()),
    //     Some("TEST 13".to_string()),
    //     Some("TEST 14".to_string()),
    //     Some("TEST 15".to_string()),
    //     Some("TEST 16".to_string()),
    //     Some("TEST 17".to_string()),
    //     Some("TEST 18".to_string()),
    //     Some("TEST 19".to_string()),
    //     Some("TEST 20".to_string()),
    // ];

    // assert_eq!(thread_manager(strings, test1), result);
    thread_manager(strings, test1);
}
