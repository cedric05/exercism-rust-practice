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
        let mut vec = self._scores.to_vec();
        vec.sort_unstable_by(|a, b| b.cmp(a));
        vec.truncate(3);
        vec
    }
}

#[test]

fn test_personal_top_three_highest_to_lowest() {
    let high_scores = HighScores::new(&[20, 10, 30]);
    assert_eq!(high_scores.personal_top_three(), vec![30, 20, 10]);
}
