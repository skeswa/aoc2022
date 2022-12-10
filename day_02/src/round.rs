use crate::hand_shape::HandShape;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

/// Represents one round of a rip roarin' game of rock paper scissors.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Round {
    /// Hand shape submitted by this player of rock paper scissors.
    pub(crate) my_hand_shape: HandShape,
    /// Hand shape submitted by the other player of rock paper scissors.
    pub(crate) opponent_hand_shape: HandShape,
}

impl Round {
    /// Returns the constructed [Round] that best corresponds to the specified
    /// `encoded_round`.
    pub(crate) fn parse(encoded_round: &str) -> Result<Round> {
        lazy_static! {
          /// Regular expression designed to match rounds encoded within a strategy
          /// guide. A round might look like `"A  X"` or `"  B Y"`.
          static ref ENCODED_ROUND_PATTERN: Regex =
              Regex::new(r"^\s*([^\s]+)\s+([^\s]+)\s*$").unwrap();
        }

        let parsed_encoded_round = ENCODED_ROUND_PATTERN
            .captures(encoded_round)
            .with_context(|| format!("\"{}\" is not a valid encoded round", encoded_round))?;

        let my_encoded_hand_shape = parsed_encoded_round.get(0).unwrap().as_str();
        let opponent_encoded_hand_shape = parsed_encoded_round.get(1).unwrap().as_str();

        let my_hand_shape = HandShape::parse_for_me(my_encoded_hand_shape)?;
        let opponent_hand_shape = HandShape::parse_for_opponent(opponent_encoded_hand_shape)?;

        Ok(Round {
            my_hand_shape: my_hand_shape,
            opponent_hand_shape: opponent_hand_shape,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_parses_valid_encoded_rounds() {
        assert_eq!(
            Round::parse("  A X").unwrap(),
            Round {
                my_hand_shape: HandShape::Paper,
                opponent_hand_shape: HandShape::Paper,
            },
        );
        assert_eq!(
            Round::parse("B\nY").unwrap(),
            Round {
                my_hand_shape: HandShape::Paper,
                opponent_hand_shape: HandShape::Paper,
            },
        );
    }

    #[test]
    fn correctly_parses_invalid_encoded_rounds() {
        assert!(Round::parse("  a X").is_err());
        assert!(Round::parse("BY").is_err());
    }

    #[test]
    fn correctly_parses_reversed_encoded_rounds() {
        assert!(Round::parse("  X A").is_err());
        assert!(Round::parse("Y\nB").is_err());
    }
}
