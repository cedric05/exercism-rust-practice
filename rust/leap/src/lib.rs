use std::ops::Rem;

pub fn is_leap_year(year: u64) -> bool {
    if year.rem(4).eq(&0) {
        if year.rem(100).eq(&0) {
            return year.rem(400).eq(&0);
        } else {
            return true;
        }
    }
    false
}
