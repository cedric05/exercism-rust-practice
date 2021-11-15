pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        vec![String::from(""); digits.len() + 1]
    } else {
        digits
            .chars()
            .collect::<Vec<char>>()
            .windows(len)
            .map(|x| {
                let mut ram = String::new();
                x.iter().for_each(|i| {
                    ram.push(*i);
                });
                ram
            })
            .collect()
    }
}
