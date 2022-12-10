use anyhow::{anyhow, Result};

/// Enumerates every usable hand shape in a rip roarin' game of rock paper
/// scissors.
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum HandShape {
    /// Hand shape signified by an open palm. Paper beats rock.
    Paper,
    /// Hand shape signified by a closed fist. Rock beats scissors.
    Rock,
    /// Hand shape signified by a closed fist with only the index and middle
    /// fingers sticking out. Rock beats scissors.
    Scissors,
}

impl HandShape {
    /// Returns the [HandShape] corresponding to the specified `encoded_round`
    /// string targeted at the main player.
    pub(crate) fn parse_for_me(encoded_round: &str) -> Result<HandShape> {
        let sanitized_encoded_round = encoded_round.trim();

        match sanitized_encoded_round {
            "A" => Ok(HandShape::Paper),
            "B" => Ok(HandShape::Rock),
            "C" => Ok(HandShape::Scissors),
            _ => Err(anyhow!(
                "\"{}\" is not a valid hand shape (for you, anyway)",
                sanitized_encoded_round,
            )),
        }
    }

    /// Returns the [HandShape] corresponding to the specified `encoded_round`
    /// string targeted at the opposing player.
    pub(crate) fn parse_for_opponent(encoded_round: &str) -> Result<HandShape> {
        let sanitized_encoded_round = encoded_round.trim();

        match sanitized_encoded_round {
            "X" => Ok(HandShape::Paper),
            "Y" => Ok(HandShape::Rock),
            "Z" => Ok(HandShape::Scissors),
            _ => Err(anyhow!(
                "\"{}\" is not a valid hand shape (for your opponent, anyway)",
                sanitized_encoded_round,
            )),
        }
    }
}
