extern crate advent;
extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod crate_move;
mod crate_piling_order;
mod crate_rearrangement_procedure;
mod crate_stacks;

use crate::crate_piling_order::CratePilingOrder;
use crate::crate_rearrangement_procedure::CrateRearrangementProcedure;
use crate::crate_stacks::Crate;
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let config = advent::begin();

    let encoded_crate_rearrangement_procedure = advent::data(&config).await?;

    let crate_rearrangement_procedure =
        CrateRearrangementProcedure::parse(&encoded_crate_rearrangement_procedure)
            .context("Failed to parse crate rearrangement procedure")?;

    let rearranged_crate_stacks = crate_rearrangement_procedure.execute(match config.part {
        1 => CratePilingOrder::Flipped,
        _ => CratePilingOrder::InOrder,
    });

    let top_crates = rearranged_crate_stacks
        .top_crates()
        .map(|maybe_top_crate| {
            maybe_top_crate.with_context(|| {
                format!(
                    "One of the crates was missing! {:?}",
                    rearranged_crate_stacks,
                )
            })
        })
        .collect::<Result<Vec<&Crate>>>()
        .context("Failed to pull out top crates")?;

    let top_crate_string = top_crates
        .iter()
        .map(|top_crate| top_crate.letter)
        .collect::<String>();

    println!("Top crate letters: {}", top_crate_string);

    Ok(())
}
