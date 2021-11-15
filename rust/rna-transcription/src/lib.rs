#[derive(Debug, PartialEq)]
pub struct Dna {
    seq: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    seq: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna
            .char_indices()
            .find(|(_, c)| !['C', 'G', 'T', 'A'].contains(c))
        {
            Some((index, _)) => Err(index),
            None => Ok(Dna {
                seq: String::from(dna),
            }),
        }
    }

    pub fn into_rna(self) -> Rna {
        let mut seq = String::from("");
        self.seq.chars().for_each(|x| {
            seq.push(match x {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => panic!(),
            })
        });
        Rna { seq }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna
            .char_indices()
            .find(|(_, c)| !['C', 'G', 'A', 'U'].contains(c))
        {
            Some((index, _)) => Err(index),
            None => Ok(Rna {
                seq: String::from(rna),
            }),
        }
    }
}
