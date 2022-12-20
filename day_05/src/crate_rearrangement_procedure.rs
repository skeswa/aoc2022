use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    crate_move::CrateMove, crate_piling_order::CratePilingOrder, crate_stacks::CrateStacks,
};

/// Represents the relocation of some number of crates between crate stacks.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct CrateRearrangementProcedure {
    /// Sequence of crate moves to apply to the crate stacks.
    pub(crate) crate_moves: Vec<CrateMove>,
    /// Starting state for the crates being rearranged.
    pub(crate) crate_stacks: CrateStacks,
}

impl CrateRearrangementProcedure {
    /// Returns the constructed [CrateRearrangementProcedure] that best
    /// corresponds to the specified `encoded_rearrangement_procedure`.
    pub(crate) fn parse(
        encoded_rearrangement_procedure: &str,
    ) -> Result<CrateRearrangementProcedure> {
        lazy_static! {
          /// Regular expression designed to match empty lines within other
          /// larger strings.
          static ref EMPTY_LINE_PATTERN: Regex =
          Regex::new(r"\n\s*\n").unwrap();
        }

        let encoded_rearrangement_procedure_halves = EMPTY_LINE_PATTERN
            .split(encoded_rearrangement_procedure)
            .collect::<Vec<&str>>();

        if encoded_rearrangement_procedure_halves.len() != 2 {
            return Err(anyhow!(
                "Encoded rearrangement procedure had the wrong number of halves ({})",
                encoded_rearrangement_procedure.len()
            ));
        }

        let crate_moves = encoded_rearrangement_procedure_halves[1]
            .lines()
            .map(|encoded_crate_move| CrateMove::parse(encoded_crate_move))
            .collect::<Result<Vec<CrateMove>>>()
            .context("Failed to read first half of rearrangement procedure")?;
        let crate_stacks = CrateStacks::parse(encoded_rearrangement_procedure_halves[0])
            .context("Failed to read first half of rearrangement procedure")?;

        Ok(CrateRearrangementProcedure {
            crate_moves: crate_moves,
            crate_stacks: crate_stacks,
        })
    }

    /// Returns the result of applying this rearrangement procedure's crate
    /// moves to a clone of its crate stacks.
    ///
    ///  * `create_piling_order` describes the piling order that should be used
    pub(crate) fn execute(&self, create_piling_order: CratePilingOrder) -> CrateStacks {
        self.crate_stacks
            .apply(&self.crate_moves, create_piling_order)
    }
}
