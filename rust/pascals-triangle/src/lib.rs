use std::iter;

pub struct PascalsTriangle {
    pub row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }
}
impl PascalsTriangle {
    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut ret = vec![];
        (1..=self.row_count).for_each(|row_index| {
            ret.push(match row_index {
                1 => vec![1; 1],
                2 => vec![1; 2],
                _ => ret
                    .get((row_index - 2) as usize)
                    .map(|prev: &Vec<u32>| {
                        iter::once(1)
                            .chain(prev.windows(2).map(|a| a[0] + a[1]))
                            .chain(iter::once(1))
                            .collect()
                    })
                    .unwrap(),
            });
        });
        ret
    }
}
