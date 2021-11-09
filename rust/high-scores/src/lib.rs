use std::cmp::Ordering;

#[derive(Debug)]
pub struct HighScores {
    _scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            _scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self._scores
    }

    pub fn latest(&self) -> Option<u32> {
        if self._scores.is_empty() {
            None
        } else {
            Some(self._scores[self._scores.len() - 1])
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        self._scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let vec = self
            ._scores
            .chunks(3)
            .fold([None, None, None], |a: [Option<u32>; 3], b| {
                let mut v: Vec<Option<u32>> =
                    Vec::from_iter(a.iter().copied().chain(b.iter().copied().map(Some)));
                v.sort_by(|x, y| {
                    if x.is_some() && y.is_some() {
                        x.cmp(y)
                    } else if x.is_some() {
                        Ordering::Greater
                    } else if y.is_some() {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                });
                [fun_name(&mut v), fun_name(&mut v), fun_name(&mut v)]
            });
        Vec::from_iter(vec.iter().flatten().copied())
    }
}

fn fun_name(v: &mut Vec<Option<u32>>) -> Option<u32> {
    v.pop().unwrap_or(None)
}

#[test]

fn test_personal_top_three_highest_to_lowest() {
    let high_scores = HighScores::new(&[20, 10, 30]);
    assert_eq!(high_scores.personal_top_three(), vec![30, 20, 10]);
}
