use std::collections::HashSet;

/*
c**2 + c**2 + 2*a*b + 2*c*a + 2*c*b = N*N

c*c + a*b + c*a + c*b = N*N/2
c* (c+a+b) + a*b = N*N/2
c*N + a*b = N*N/2
c = N/2 - (a*b)/N

*/
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut set = HashSet::new();
    for a in 1..=(sum / 3) {
        // if sum % a == 0 {
        // print!("{}", a);
        for b in a + 1..(((sum - a) / 2) + 1) {
            let c = sum - a - b;
            if c * c == a * a + b * b {
                set.insert([a, b, c]);
            }
        }
        // }
    }
    set
}

#[test]
fn test_1000() {
    println!("{:?}", find(1000))
}
