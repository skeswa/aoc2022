use anyhow::{anyhow, Context, Result};

use lazy_static::lazy_static;
use regex::Regex;

/// Enumerates every variety of rope move.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RopeMove {
    /// How the head of the rope should move.
    direction: RopeMoveDirection,
    /// Magnitude of this rope move.
    distance: usize,
}

impl RopeMove {
    /// Attempts to interpret `encoded_rope_move` as a [RopeMove], returning
    /// [Err] if interpretation fails.
    pub(crate) fn parse(encoded_rope_move: &str) -> Result<RopeMove> {
        lazy_static! {
          /// Regular expression designed to match strings that look like
          /// " R 3 " or "L  12"
          static ref ENCODED_ROPE_MOVE_PATTERN: Regex =
              Regex::new(r"^\s*([a-zA-Z]+)\s+(\d+)\s*$").unwrap();
        }

        let (raw_direction, raw_distance) = ENCODED_ROPE_MOVE_PATTERN
            .captures(encoded_rope_move)
            .and_then(|captures| {
                if let Some(first_group) = captures.get(1) {
                    if let Some(second_group) = captures.get(2) {
                        return Some((first_group.as_str(), second_group.as_str()));
                    }
                }

                None
            })
            .with_context(|| format!("\"{}\" is not a valid rope move", encoded_rope_move))?;

        let direction = RopeMoveDirection::parse(raw_direction)?;
        let distance = raw_distance
            .parse::<usize>()
            .with_context(|| format!("\"{}\" is not a valid rope move distance", raw_distance))?;

        Ok(RopeMove {
            direction: direction,
            distance: distance,
        })
    }
}

/// Enumerates every variety of rope move direction.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum RopeMoveDirection {
    /// Head of the rope should move northward.
    Down,
    /// Head of the rope should move westward.
    Left,
    /// Head of the rope should move eastward.
    Right,
    /// Head of the rope should move southward.
    Up,
}

impl RopeMoveDirection {
    /// Attempts to interpret `encoded_rope_move_direction` as a
    /// [RopeMoveDirection], returning [Err] if interpretation fails.
    pub(crate) fn parse(encoded_rope_move_direction: &str) -> Result<RopeMoveDirection> {
        match encoded_rope_move_direction.trim().to_uppercase().as_str() {
            "D" => Ok(RopeMoveDirection::Down),
            "L" => Ok(RopeMoveDirection::Left),
            "R" => Ok(RopeMoveDirection::Right),
            "U" => Ok(RopeMoveDirection::Up),
            _ => Err(anyhow!(
                "\"{}\" is not a valid ropoe move direction",
                encoded_rope_move_direction
            )),
        }
    }
}
