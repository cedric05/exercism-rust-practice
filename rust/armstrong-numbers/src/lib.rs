pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u32> = vec![];
    let mut residue = num;
    loop {
        if residue / 10 < 1 {
            digits.push(residue);
            break;
        } else {
            digits.push(residue % 10);
            residue /= 10;
        }
    }
    let num_of_digits = digits.len() as u32;
    digits
        .into_iter()
        .map(|i| i.pow(num_of_digits))
        .sum::<u32>()
        .eq(&num)
}
