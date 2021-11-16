pub fn number(user_number: &str) -> Option<String> {
    let mut ret = user_number
        .chars()
        .rev()
        .filter(|x| x.is_digit(10))
        .take(11)
        .collect::<Vec<char>>();
    let len = ret.len();
    if [10, 11].contains(&len) {
        ret.reverse();
        // let from_iter = String::from_iter(ret);
        let from_iter = ret;
        let (country_code, area_code, exchange_code, subscriber_code) = if from_iter.len() == 11 {
            (
                from_iter[0],
                &from_iter[1..4],
                &from_iter[4..7],
                &from_iter[7..],
            )
        } else {
            ('1', &from_iter[0..3], &from_iter[3..6], &from_iter[6..])
        };
        let invalid_start = [Some(&'1'), Some(&'0')];
        if invalid_start.contains(&area_code.first())
            || invalid_start.contains(&exchange_code.first())
            || country_code != '1'
        {
            None
        } else {
            let from_iter = String::from_iter(
                area_code
                    .iter()
                    .chain(exchange_code.iter())
                    .chain(subscriber_code.iter()),
            );
            Some(from_iter)
        }
    } else {
        None
    }
}
