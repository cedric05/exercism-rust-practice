pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        None
    } else {
        let start = 0;
        let end = array.len() - 1;
        find_between(array, key, start, end)
    }
}

fn find_between(array: &[i32], key: i32, start: usize, end: usize) -> Option<usize> {
    if key < array[start] || key > array[end] {
        None
    } else if array[start] == key {
        Some(start)
    } else if array[end] == key {
        Some(end)
    } else if start == end || start == end - 1 {
        None
    } else {
        let mid = (start + end) / 2;
        if key <= array[mid] {
            find_between(array, key, start, mid)
        } else {
            find_between(array, key, mid, end)
        }
    }
}
