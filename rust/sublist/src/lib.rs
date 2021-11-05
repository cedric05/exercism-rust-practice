#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    match (a.len(), b.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (m, n) if m > n => {
            if a.windows(b.len()).any(|arr| arr == b) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        (m, n) if m < n => {
            if b.windows(a.len()).any(|arr| arr == a) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        (_, _) => {
            if a == b {
                Equal
            } else {
                Unequal
            }
        }
    }
}
