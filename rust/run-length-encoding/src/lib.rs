pub fn encode(source: &str) -> String {
    let mut count: u32 = 1;
    let mut iter = source.chars();
    let mut ret = String::from("");
    match iter.next() {
        Some(prev_char) => {
            let mut prev_char = prev_char;
            for i in iter {
                if i == prev_char {
                    count += 1;
                } else {
                    if count > 1 {
                        ret.push_str(&format!("{}{}", count, prev_char));
                    } else {
                        ret.push(prev_char);
                    }
                    prev_char = i;
                    count = 1;
                }
            }
            if count > 1 {
                ret.push_str(&format!("{}{}", count, prev_char));
            } else {
                ret.push(prev_char);
            }
            ret
        }
        None => String::new(),
    }
}

pub fn decode(source: &str) -> String {
    let mut iter = source.chars();
    let mut ret = String::new();
    loop {
        let mut temp = String::new();
        loop {
            match iter.next().map(|x| (x.is_digit(10), x)) {
                Some((true, c)) => temp.push(c),
                Some((false, c)) => match temp.parse() {
                    Ok(parsed) => {
                        std::iter::repeat(c).take(parsed).for_each(|_| {
                            ret.push(c);
                        });
                        temp = String::new();
                    }
                    Err(_) => {
                        ret.push(c);
                    }
                },
                None => {
                    return ret;
                }
            }
        }
    }
}

#[test]
fn test_normal() {
    assert_eq!("AABBB", decode("2A3B"));
    assert_eq!("AAAAAAAAAABBB", decode("10A3B"));
}

#[test]
fn test_normal2() {
    assert_eq!("2A3B", encode("AABBB"));
    assert_eq!("10A3B", encode("AAAAAAAAAABBB"));
}
