use std::collections::HashMap;
use std::ops::ControlFlow::Break;
use std::ops::ControlFlow::Continue;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => {
            nucleotide_counts(dna).map(|x| x.get(&nucleotide).copied().unwrap_or(0))
        }
        _ => Err(nucleotide),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    match dna
        .chars()
        .try_fold((0, 0, 0, 0), |(a, c, g, t), n| match n {
            'A' => Continue((a + 1, c, g, t)),
            'C' => Continue((a, c + 1, g, t)),
            'G' => Continue((a, c, g + 1, t)),
            'T' => Continue((a, c, g, t + 1)),
            _ => Break(n),
        }) {
        Continue((a, c, g, t)) => Ok(HashMap::from([('A', a), ('C', c), ('G', g), ('T', t)])),
        Break(c) => Err(c),
    }
}
