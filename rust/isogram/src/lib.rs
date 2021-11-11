use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set: HashSet<char> = HashSet::new();
    candidate
        .replace("-", "")
        .replace(" ", "")
        .to_lowercase()
        .chars()
        .all(|x| set.insert(x))
}

#[test]
fn test_simple_str() {
    assert!(check("ram"));
    assert!(!check("rama"));
    assert!(check("ram-esh"));
    assert!(check("ram--"));
    assert!(!check("ram ram"));
    assert!(check("ram he lk"));
    assert!(check("downstream"));
    assert!(check("six-year-old"));
    assert!(check("background"));
    assert!(check("lumberjacks"));
}
