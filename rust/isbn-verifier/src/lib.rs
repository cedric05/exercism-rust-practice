use std::ops::Rem;

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // (x1 * 10 + x2 * 9 + x3 * 8 + x4 * 7 + x5 * 6 + x6 * 5 + x7 * 4 + x8 * 3 + x9 * 2 + x10 * 1) mod 11 == 0

    let mut num_count = 0;
    let sum = isbn
        .chars()
        .filter(|c| c == &'X' || c.is_digit(10))
        .enumerate()
        .filter(|(index, x)| {
            if index == &9 {
                x.is_digit(10) || x == &'X'
            } else {
                x.is_digit(10)
            }
        })
        .map(|(_, x)| match x {
            '0'..='9' => x.to_digit(10).unwrap(),
            'X' => 10,
            _ => panic!("this should not happen"),
        })
        .enumerate()
        .map(|(index, x)| (index as u32 + 1) * x)
        .inspect(|_| {
            num_count += 1;
        })
        .sum::<u32>();
    num_count == 10 && sum != 0 && sum.rem(11).eq(&0)
}
