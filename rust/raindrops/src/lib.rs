pub fn raindrops(n: u32) -> String {
    let is_factor = |x| n % x == 0;
    let mut s = vec![];
    if is_factor(3) {
        s.push("Pling");
    }
    if is_factor(5) {
        s.push("Plang");
    }
    if is_factor(7) {
        s.push("Plong");
    }
    if s.is_empty() {
        n.to_string()
    } else {
        s.join("")
    }
}

#[test]
fn test_haha() {
    assert_eq!("Plang", raindrops(10));
    assert_eq!("PlingPlang", raindrops(15));
    assert_eq!("11", raindrops(11));
    assert_eq!("Plong", raindrops(7));
}
