use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};

/// Represents the relocation of some number of crates between crate stacks.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct CrateMove {
    /// Index of the stack that should receive `number_of_crates` crates.
    pub(crate) destination_crate_index: usize,
    /// How many crates will move.
    pub(crate) number_of_crates: usize,
    /// Index of the stack that should lose `number_of_crates` crates.
    pub(crate) origin_crate_index: usize,
}

impl CrateMove {
    /// Returns the constructed [CrateMove] that best corresponds to the
    /// specified `encoded_crate_move`.
    pub(crate) fn parse(encoded_crate_move: &str) -> Result<CrateMove> {
        lazy_static! {
          /// Regular expression designed to match strings that look like
          /// "move\t1   from 2 to 1 ".
          static ref ENCODED_CRATE_MOVE_PATTERN: Regex =
          RegexBuilder::new(r"^\s*move\s+(\d+)\s+from\s+(\d+)\s+to\s+(\d+)\s*$").case_insensitive(true).build().unwrap();
        }

        let parsed_encoded_crate_move = ENCODED_CRATE_MOVE_PATTERN
            .captures(encoded_crate_move)
            .with_context(|| {
                format!(
                    "\"{}\" is not a valid encoded crate move",
                    encoded_crate_move
                )
            })?;

        let first_chunk = parsed_encoded_crate_move
            .get(1)
            .with_context(|| {
                format!(
                    "Failed to extract the first capture group from \"{}\"",
                    encoded_crate_move
                )
            })?
            .as_str();
        let second_chunk = parsed_encoded_crate_move
            .get(2)
            .with_context(|| {
                format!(
                    "Failed to extract the second capture group from \"{}\"",
                    encoded_crate_move
                )
            })?
            .as_str();
        let third_chunk = parsed_encoded_crate_move
            .get(3)
            .with_context(|| {
                format!(
                    "Failed to extract the third capture group from \"{}\"",
                    encoded_crate_move
                )
            })?
            .as_str();

        let destination_crate_number = third_chunk
            .parse::<usize>()
            .with_context(|| format!("Failed to parse \"{}\" as an integer", third_chunk))?;
        let number_of_crates = first_chunk
            .parse::<usize>()
            .with_context(|| format!("Failed to parse \"{}\" as an integer", first_chunk))?;
        let origin_crate_number = second_chunk
            .parse::<usize>()
            .with_context(|| format!("Failed to parse \"{}\" as an integer", second_chunk))?;

        if destination_crate_number < 1 {
            return Err(anyhow!(
                "detination crate number, {}, is out of range",
                destination_crate_number
            ));
        }
        if origin_crate_number < 1 {
            return Err(anyhow!(
                "origin crate number, {}, is out of range",
                origin_crate_number
            ));
        }

        Ok(CrateMove {
            destination_crate_index: destination_crate_number - 1,
            number_of_crates: number_of_crates,
            origin_crate_index: origin_crate_number - 1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works_given_ideal_input() {
        assert_eq!(
            CrateMove::parse("move 1 from 2 to 1").unwrap(),
            CrateMove {
                destination_crate_index: 0,
                number_of_crates: 1,
                origin_crate_index: 1,
            },
        );
    }

    #[test]
    fn parse_works_given_nonideal_all_caps_input() {
        assert_eq!(
            CrateMove::parse("MOVE 12 FROM 8 TO 9").unwrap(),
            CrateMove {
                destination_crate_index: 8,
                number_of_crates: 12,
                origin_crate_index: 7,
            },
        );
    }
}
