pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut range = (2..3).chain(3..(n + 1));
    if let Some(x) = (range).find(|x| n % x == 0) {
        result.push(x);
        result.append(&mut factors(n / x));
    }
    result
}
