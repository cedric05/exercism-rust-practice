pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let ceil = upper_bound;
    let mut v: Vec<u64> = vec![];
    (2..=ceil).for_each(|x| {
        let mut pr = 2;
        while pr * x <= upper_bound {
            v.push(pr * x);
            pr += 1;
        }
    });
    let mut ret = vec![];
    for i in 2..=ceil {
        if !v.contains(&i) {
            ret.push(i);
        }
    }
    ret
}
