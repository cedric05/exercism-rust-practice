use std::cmp;
#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
enum Includes {
    Yes,
    No,
}
pub fn sublist<T: PartialEq>(first_list: &[T], seconnd_list: &[T]) -> Comparison {
    use Comparison::*;
    use Includes::*;
    match first_list.len().cmp(&seconnd_list.len()) {
        cmp::Ordering::Less => match first_contains_second(first_list, seconnd_list) {
            Yes => Sublist,
            No => Unequal,
        },
        cmp::Ordering::Equal => match first_contains_second(first_list, seconnd_list) {
            Yes => Equal,
            No => Unequal,
        },
        cmp::Ordering::Greater => match first_contains_second(seconnd_list, first_list) {
            Yes => Superlist,
            No => Equal,
        },
    }
}
fn first_contains_second<T: std::cmp::PartialEq>(l1: &[T], l2: &[T]) -> Includes {
    if l1.is_empty() {
        return Includes::Yes;
    }
    match l2.windows(l1.len()).any(|arr| arr == l1) {
        true => Includes::Yes,
        false => Includes::No,
    }
}
