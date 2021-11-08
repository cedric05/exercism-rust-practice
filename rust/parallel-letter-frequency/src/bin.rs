use parallel_letter_frequency::frequency;

fn main() {
    let out = frequency(&["abc"; 1000], 10);

    print!("{:?}", out);
}
