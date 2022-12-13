use std::cmp::Ordering;

use crate::scorable::Scorable;
use anyhow::{anyhow, Result};

/// Enumerates every usable hand shape in a rip roarin' game of rock paper
/// scissors.
#[derive(Debug, Eq, PartialEq, PartialOrd)]
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
            "X" => Ok(HandShape::Rock),
            "Y" => Ok(HandShape::Paper),
            "Z" => Ok(HandShape::Scissors),
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
            "A" => Ok(HandShape::Rock),
            "B" => Ok(HandShape::Paper),
            "C" => Ok(HandShape::Scissors),
            _ => Err(anyhow!(
                "\"{}\" is not a valid hand shape (for your opponent, anyway)",
                sanitized_encoded_round,
            )),
        }
    }

    /// Returns the [HandShape] that this [HandShape] beats.
    pub(crate) fn loser(&self) -> HandShape {
        match self {
            HandShape::Paper => HandShape::Paper,
            HandShape::Rock => HandShape::Scissors,
            HandShape::Scissors => HandShape::Paper,
        }
    }

    /// Returns the [HandShape] that beats this [HandShape].
    pub(crate) fn winner(&self) -> HandShape {
        match self {
            HandShape::Paper => HandShape::Scissors,
            HandShape::Rock => HandShape::Paper,
            HandShape::Scissors => HandShape::Rock,
        }
    }
}

impl Scorable for HandShape {
    fn score(&self) -> u32 {
        match self {
            Self::Paper => 2,
            Self::Rock => 1,
            Self::Scissors => 3,
        }
    }
}

impl Ord for HandShape {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Paper => match other {
                Self::Paper => Ordering::Equal,
                Self::Rock => Ordering::Greater,
                Self::Scissors => Ordering::Less,
            },
            Self::Rock => match other {
                Self::Paper => Ordering::Less,
                Self::Rock => Ordering::Equal,
                Self::Scissors => Ordering::Greater,
            },
            Self::Scissors => match other {
                Self::Paper => Ordering::Greater,
                Self::Rock => Ordering::Less,
                Self::Scissors => Ordering::Equal,
            },
        }
    }
}