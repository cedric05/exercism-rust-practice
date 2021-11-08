use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::vec;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let t: Vec<String> = input
        .iter()
        .map(|a| String::from(*a).to_lowercase())
        .collect();
    let str_list = Arc::new(Mutex::new(t));
    let ret = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    for _ in 0..worker_count {
        let input = Arc::clone(&str_list);
        let ret = Arc::clone(&ret);
        let handle = thread::spawn(move || loop {
            if let Ok(ref mut v) = input.try_lock() {
                if let Ok(ref mut map) = ret.try_lock() {
                    if let Some(s) = v.pop() {
                        s.chars().filter(|c| c.is_alphabetic()).for_each(|x| {
                            match map.get_mut(&x) {
                                Some(c) => {
                                    *c += 1;
                                }
                                None => {
                                    map.insert(x, 1);
                                }
                            }
                        });
                    } else {
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mutex_guard = ret.lock().unwrap().to_owned();
    mutex_guard
}
