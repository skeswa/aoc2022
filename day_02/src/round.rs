use std::cmp::Ordering;
use std::fmt::Debug;

use crate::{
    hand_shape::HandShape, round_outcome::RoundOutcome, scorable::Scorable,
    strategy_guide_interpretation::StrategyGuideInterpretation,
};
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

/// Represents one round of a rip roarin' game of rock paper scissors.
#[derive(Eq, PartialEq)]
pub(crate) struct Round {
    /// Hand shape submitted by this player of rock paper scissors.
    pub(crate) my_hand_shape: HandShape,
    /// Hand shape submitted by the other player of rock paper scissors.
    pub(crate) opponent_hand_shape: HandShape,
}

impl Round {
    /// Returns the constructed [Round] that best corresponds to the specified
    /// `encoded_round`.
    ///
    /// * `interpretation` configures how this method should interpret
    ///   `encoded_round`
    pub(crate) fn parse(
        encoded_round: &str,
        interpretation: StrategyGuideInterpretation,
    ) -> Result<Round> {
        lazy_static! {
          /// Regular expression designed to match rounds encoded within a strategy
          /// guide. A round might look like `"A  X"` or `"  B Y"`.
          static ref ENCODED_ROUND_PATTERN: Regex =
              Regex::new(r"^\s*([^\s]+)\s+([^\s]+)\s*$").unwrap();
        }

        let parsed_encoded_round = ENCODED_ROUND_PATTERN
            .captures(encoded_round)
            .with_context(|| format!("\"{}\" is not a valid encoded round", encoded_round))?;

        let first_chunk = parsed_encoded_round.get(1).unwrap().as_str();
        let second_chunk = parsed_encoded_round.get(2).unwrap().as_str();

        let opponent_hand_shape = HandShape::parse_for_opponent(first_chunk)?;

        let my_hand_shape = match interpretation {
            StrategyGuideInterpretation::HandShape => HandShape::parse_for_me(second_chunk),
            StrategyGuideInterpretation::RoundOutcome => {
                let round_outcome = RoundOutcome::parse(second_chunk)?;

                Ok(HandShape::to_achieve(&round_outcome, &opponent_hand_shape))
            }
        }?;

        Ok(Round {
            my_hand_shape: my_hand_shape,
            opponent_hand_shape: opponent_hand_shape,
        })
    }

    /// Derives the outcome of this [Round].
    pub(crate) fn outcome(&self) -> RoundOutcome {
        match self.my_hand_shape.cmp(&self.opponent_hand_shape) {
            Ordering::Equal => RoundOutcome::Draw,
            Ordering::Greater => RoundOutcome::Win,
            Ordering::Less => RoundOutcome::Loss,
        }
    }
}

impl Debug for Round {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Round")
            .field("my_hand_shape", &self.my_hand_shape)
            .field("opponent_hand_shape", &self.opponent_hand_shape)
            .field("outcome", &self.outcome())
            .field("score", &self.score())
            .finish()
    }
}

impl Scorable for Round {
    fn score(&self) -> u32 {
        self.my_hand_shape.score() + self.outcome().score()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_parses_valid_encoded_rounds() {
        assert_eq!(
            Round::parse("  A X", StrategyGuideInterpretation::HandShape).unwrap(),
            Round {
                my_hand_shape: HandShape::Rock,
                opponent_hand_shape: HandShape::Rock,
            },
        );
        assert_eq!(
            Round::parse("  A X", StrategyGuideInterpretation::RoundOutcome).unwrap(),
            Round {
                my_hand_shape: HandShape::Scissors,
                opponent_hand_shape: HandShape::Rock,
            },
        );

        assert_eq!(
            Round::parse("B\nY", StrategyGuideInterpretation::HandShape).unwrap(),
            Round {
                my_hand_shape: HandShape::Paper,
                opponent_hand_shape: HandShape::Paper,
            },
        );
        assert_eq!(
            Round::parse("B\nY", StrategyGuideInterpretation::RoundOutcome).unwrap(),
            Round {
                my_hand_shape: HandShape::Paper,
                opponent_hand_shape: HandShape::Paper,
            },
        );

        assert_eq!(
            Round::parse("A\nZ", StrategyGuideInterpretation::HandShape).unwrap(),
            Round {
                my_hand_shape: HandShape::Scissors,
                opponent_hand_shape: HandShape::Rock,
            },
        );
        assert_eq!(
            Round::parse("A\nZ", StrategyGuideInterpretation::RoundOutcome).unwrap(),
            Round {
                my_hand_shape: HandShape::Paper,
                opponent_hand_shape: HandShape::Rock,
            },
        );
    }

    #[test]
    fn correctly_parses_invalid_encoded_rounds() {
        assert!(Round::parse("  a X", StrategyGuideInterpretation::HandShape).is_err());
        assert!(Round::parse("  a X", StrategyGuideInterpretation::RoundOutcome).is_err());

        assert!(Round::parse("BY", StrategyGuideInterpretation::HandShape).is_err());
        assert!(Round::parse("BY", StrategyGuideInterpretation::RoundOutcome).is_err());
    }

    #[test]
    fn correctly_parses_reversed_encoded_rounds() {
        assert!(Round::parse("  X A", StrategyGuideInterpretation::HandShape).is_err());
        assert!(Round::parse("  X A", StrategyGuideInterpretation::RoundOutcome).is_err());

        assert!(Round::parse("Y\nB", StrategyGuideInterpretation::HandShape).is_err());
        assert!(Round::parse("Y\nB", StrategyGuideInterpretation::RoundOutcome).is_err());
    }
}
