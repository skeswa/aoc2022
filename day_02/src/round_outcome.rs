use crate::scorable::Scorable;
use anyhow::{anyhow, Result};

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

impl RoundOutcome {
    /// Returns the [RoundOutcome] that best corresponds to the specified
    /// `encoded_round_outcome`.
    pub(crate) fn parse(encoded_round_outcome: &str) -> Result<RoundOutcome> {
        match encoded_round_outcome {
            "X" => Ok(RoundOutcome::Loss),
            "Y" => Ok(RoundOutcome::Draw),
            "Z" => Ok(RoundOutcome::Win),
            _ => Err(anyhow!(
                "\"{}\" is not a valid round outcome",
                encoded_round_outcome
            )),
        }
    }
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
