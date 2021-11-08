struct PossiblePrimes {
    is_first: bool,
    prev: u32,
}

impl PossiblePrimes {
    fn new() -> PossiblePrimes {
        PossiblePrimes {
            is_first: true,
            prev: 1,
        }
    }
}

impl Iterator for PossiblePrimes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            Some(2)
        } else {
            self.prev += 2;
            Some(self.prev)
        }
    }
}

pub fn is_prime(n: u32, primes: &mut Vec<u32>) -> bool {
    if primes.contains(&n) {
        true
    } else {
        let max_check = (n as f32).sqrt().ceil() as usize;
        for i in primes.iter().take(max_check) {
            if n % i == 0 {
                return false;
            }
        }
        primes.push(n);
        true
    }
}

pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2, 3, 5, 7, 11];
    PossiblePrimes::new()
        .filter(|x| -> bool { is_prime(*x, &mut primes) })
        .take((n + 1) as usize)
        .last()
        .unwrap()
}

fn main() {
    let nth = nth(100);
    println!("{}", nth);
}
