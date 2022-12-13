use crate::scorable::Scorable;

/// Enumerates every outcome for the main player of a particular [Round].
#[derive(Debug)]
pub(crate) enum RoundOutcome {
    /// Outcome that results from both players playing the same hand shape.
    Draw,
    /// Outcome that results from the opposing player playing a hand shape
    /// that beats the main player's hand shape.
    Loss,
    /// Outcome that results from the main player playing a hand shape
    /// that beats the opposing player's hand shape.
    Win,
}

impl Scorable for RoundOutcome {
    fn score(&self) -> u32 {
        match self {
            Self::Draw => 3,
            Self::Loss => 0,
            Self::Win => 6,
        }
    }
}
