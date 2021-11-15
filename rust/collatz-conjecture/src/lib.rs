pub fn collatz_sub(n: u64) -> Option<u64> {
    let mut n: u64 = n;
    let mut count = 0;
    loop {
        if n == 1 {
            return Some(count);
        } else if n == 0 {
            return None;
        } else if n % 2 == 0 {
            count += 1;
            n /= 2;
        } else {
            match n.checked_mul(3) {
                Some(res) => match res.checked_add(1) {
                    Some(res) => {
                        count += 1;
                        n = res;
                    }
                    None => return None,
                },
                None => return None,
            }
        }
    }
}

pub fn collatz(n: u64) -> Option<u64> {
    collatz_sub(n)
}
