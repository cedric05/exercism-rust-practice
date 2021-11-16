use std::ops::Rem;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T> Luhn for T
where
    T: ToString,
{
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}

pub fn is_valid(str: String) -> bool {
    let code: Option<Vec<u32>> = str
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10))
        .collect();
    if let Some(code) = code {
        if code.len() < 2 {
            return false;
        }
        // drops int memory one by one
        return code
            .into_iter()
            .rev()
            .enumerate()
            .map(|(index, d)| match index % 2 {
                0 => d,
                _ if d * 2 > 9 => d * 2 - 9,
                _ => d * 2,
            })
            .sum::<u32>()
            .rem(10)
            .eq(&0);
    }
    false
}
