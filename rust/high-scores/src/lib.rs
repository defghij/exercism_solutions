#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl HighScores<'_> {
    pub fn new(scores: &[u32]) -> HighScores {
        HighScores { scores: scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied() // Option<&mut T> => Option<T>
    }

    pub fn personal_best(&self) -> Option<u32> {
        // Should be:
        // self.scores.iter().max().copied()

        if self.scores.len() == 0 {
            return Option::None::<u32>;
        }

        Some(self.scores.iter().fold(0u32, |mut max, &n| {
            if max < n {
                max = n;
            }
            max
        }))
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three = Vec::with_capacity(self.scores.len());
        top_three.extend_from_slice(self.scores); // because length != capacity

        //Could be reduced to:
        // top_three.iter().rev().copied().take(3).collect()

        top_three.sort();
        top_three.reverse();
        while top_three.len() > 3 {
            top_three.pop();
        }

        top_three
    }
}
