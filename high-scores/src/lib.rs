#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    #[must_use]
    pub const fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    #[must_use]
    pub const fn scores(&self) -> &[u32] {
        self.scores
    }

    #[must_use]
    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    #[must_use]
    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    #[must_use]
    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores_copy = self.scores.to_vec();
        scores_copy.sort_by(|a, b| b.cmp(a));
        scores_copy[0..std::cmp::min(scores_copy.len(), 3)].to_vec()
    }
}
