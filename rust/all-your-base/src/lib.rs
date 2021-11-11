use std::iter::from_fn;
#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        Err(Error::InvalidInputBase)
    } else if to_base <= 1 {
        Err(Error::InvalidOutputBase)
    } else {
        number
            .iter()
            .rev()
            .enumerate()
            .map(|(index, place)| {
                if *place >= from_base {
                    Err(Error::InvalidDigit(*place))
                } else {
                    Ok(from_base.pow(index as u32) * place)
                }
            })
            .collect::<Result<Vec<u32>, Error>>()
            .map(|v| {
                let mut n: u32 = v.iter().sum();
                match n {
                    0 => vec![0],
                    _ => {
                        let mut collect = from_fn(move || {
                            if n > 0 {
                                let num = n;
                                n /= to_base;
                                Some(num % to_base)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<u32>>();
                        collect.reverse();
                        collect
                    }
                }
            })
    }
}
