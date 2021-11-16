use std::ops::Rem;
pub struct Luhn {
    num: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let code: Option<Vec<u32>> = self
            .num
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
}

// impl<'a> From<&'a str> for Luhn {
//     fn from(input: &'a str) -> Self {
//         Luhn {
//             num: String::from(input),
//         }
//     }
// }

// impl From<String> for Luhn {
//     fn from(input: String) -> Self {
//         Luhn { num: input }
//     }
// }

impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Luhn {
            num: input.to_string(),
        }
    }
}
