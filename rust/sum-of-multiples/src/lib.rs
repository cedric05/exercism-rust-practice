pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|x| factors.iter().filter(|x| x != &&0).any(|f| x % f == 0))
        .sum()
}

#[test]
fn test_20() {
    assert_eq!(78, sum_of_multiples(20, &[3, 5]))
}
