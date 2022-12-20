use anyhow::{Context, Ok, Result};
use lazy_static::lazy_static;
use regex::Regex;

use crate::{crate_move::CrateMove, crate_piling_order::CratePilingOrder};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CrateStacks(Vec<CrateStack>);

impl CrateStacks {
    /// Returns the constructed [CrateStacks] instance that best corresponds to
    /// the specified `encoded_crate_stacks`.
    pub(crate) fn parse(encoded_crate_stacks: &str) -> Result<CrateStacks> {
        lazy_static! {
          /// Regular expression designed to match the numbers at the bottom
          /// of the encoded crate stacks.
          static ref LABEL_ROW_PATTERN: Regex =
              Regex::new(r"\n(?:\s+(\d+))+\s*").unwrap();
        }

        let label_row_start_index = LABEL_ROW_PATTERN
            .find(encoded_crate_stacks)
            .context("Failed to find the crate labels row")?
            .range()
            .start;

        let crate_rows = &encoded_crate_stacks[..label_row_start_index];

        let crates = crate_rows
            .lines()
            .map(|crate_row| {
                crate_row
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(4)
                    .map(|crate_chunk| crate_chunk.iter().collect::<String>())
                    .map(|encoded_crate| Crate::parse(&encoded_crate))
                    .collect::<Result<Vec<Option<Crate>>>>()
            })
            .collect::<Result<Vec<Vec<Option<Crate>>>>>()
            .context("Failed to chunkify crate rows")?;

        let row_count = crates.len();
        let column_count = if row_count >= 1 { crates[0].len() } else { 0 };

        if crates.len() < 1 {
            return Ok(CrateStacks(vec![]));
        }

        let crate_stacks = (0..column_count)
            .map(|column_index| {
                (0..row_count)
                    .rev()
                    .map(|row_index| crates[row_index][column_index])
                    .take_while(|maybe_crate| maybe_crate.is_some())
                    .map(|maybe_crate| maybe_crate.unwrap())
                    .collect::<Vec<Crate>>()
            })
            .map(|crate_column| CrateStack(crate_column))
            .collect::<Vec<CrateStack>>();

        Ok(CrateStacks(crate_stacks))
    }

    /// Applies the specified `crate_moves` to a clone of this [CrateStacks].
    ///
    ///  * `create_piling_order` describes the piling order that should be used
    pub(crate) fn apply(
        &self,
        crate_moves: &Vec<CrateMove>,
        create_piling_order: CratePilingOrder,
    ) -> CrateStacks {
        let mut crate_stacks = self.0.clone();

        for crate_move in crate_moves {
            let moved_crates =
                crate_stacks[crate_move.origin_crate_index].pop(crate_move.number_of_crates);

            crate_stacks[crate_move.destination_crate_index]
                .pile_on(moved_crates, &create_piling_order);
        }

        CrateStacks(crate_stacks)
    }

    /// Returns the [Crate] at the top of each stack.
    pub(crate) fn top_crates(&self) -> impl Iterator<Item = Option<&Crate>> {
        self.0.iter().map(|stacked_crate| stacked_crate.top_crate())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CrateStack(Vec<Crate>);

impl CrateStack {
    /// Returns the [Crate] at the top of this stack.
    ///
    ///  * `create_piling_order` describes the piling order that should be used
    pub(crate) fn pile_on(&mut self, crates: Vec<Crate>, create_piling_order: &CratePilingOrder) {
        match create_piling_order {
            CratePilingOrder::Flipped => {
                for piled_on_crate in crates.iter().rev() {
                    self.0.push(*piled_on_crate);
                }
            }
            CratePilingOrder::InOrder => {
                for piled_on_crate in crates.iter() {
                    self.0.push(*piled_on_crate);
                }
            }
        }
    }

    /// Removes `number_of_crates` from the top of this stack, and returns them.
    pub(crate) fn pop(&mut self, number_of_crates: usize) -> Vec<Crate> {
        let first_popped_index = self.0.len() - number_of_crates;

        self.0.drain(first_popped_index..).collect::<Vec<Crate>>()
    }

    /// Returns the [Crate] at the top of this stack.
    pub(crate) fn top_crate(&self) -> Option<&Crate> {
        self.0.last()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Crate {
    /// Single-character "name" of this [Crate].
    pub(crate) letter: char,
}

impl Crate {
    /// Returns the constructed [Crate] that best corresponds to the specified
    /// `encoded_crate`.
    pub(crate) fn parse(encoded_crate: &str) -> Result<Option<Crate>> {
        lazy_static! {
          /// Regular expression designed to match strings that look like
          /// "[a]" or " [B]\n".
          static ref ENCODED_CRATE_PATTERN: Regex =
              Regex::new(r"^\s*\[([a-zA-Z])\]\s*$").unwrap();

            /// Regular expression designed to match strings are pure whitespace
          static ref WHITESPACE_PATTERN: Regex =
              Regex::new(r"^\s+$").unwrap();
        }

        if WHITESPACE_PATTERN.is_match(encoded_crate) {
            return Ok(None);
        }

        let parsed_encoded_crate = ENCODED_CRATE_PATTERN
            .captures(encoded_crate)
            .with_context(|| format!("\"{}\" is not a valid encoded crate", encoded_crate))?;

        let letter = parsed_encoded_crate
            .get(1)
            .with_context(|| {
                format!(
                    "Failed to extract the first capture group from \"{}\"",
                    encoded_crate
                )
            })?
            .as_str()
            .chars()
            .next()
            .with_context(|| {
                format!("Failed to read the crate letter from \"{}\"", encoded_crate)
            })?;

        Ok(Some(Crate { letter: letter }))
    }
}
