extern crate advent;
extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod section_assignment;
mod section_assignment_pair;

use anyhow::{anyhow, Context, Result};
use section_assignment_pair::SectionAssignmentPair;

#[tokio::main]
async fn main() -> Result<()> {
    let config = advent::begin();

    let encoded_section_assignment_pairs = advent::data(&config).await?;

    let section_assignment_pairs = encoded_section_assignment_pairs
        .lines()
        .map(|encoded_section_assignment_pair| {
            SectionAssignmentPair::parse(encoded_section_assignment_pair)
        })
        .collect::<Result<Vec<SectionAssignmentPair>>>()
        .context("Failed to parse encoded section assignment pairs")?;

    match config.part {
        1 => {
            let section_assignment_pairs_with_redundancy = section_assignment_pairs
                .iter()
                .filter(|section_assignment_pair| section_assignment_pair.has_rendundancy())
                .collect::<Vec<&SectionAssignmentPair>>();

            println!(
                "Section assignment pairs with redundancy: {}\n{:?}",
                section_assignment_pairs_with_redundancy.len(),
                section_assignment_pairs_with_redundancy
            );

            Ok(())
        }
        2 => {
            let overlapping_section_assignment_pairs = section_assignment_pairs
                .iter()
                .filter(|section_assignment_pair| section_assignment_pair.has_overlap())
                .collect::<Vec<&SectionAssignmentPair>>();

            println!(
                "Overlapping section assignment pairs: {}\n{:?}",
                overlapping_section_assignment_pairs.len(),
                overlapping_section_assignment_pairs,
            );

            Ok(())
        }
        _ => Err(anyhow!("Part {} is not implemented", config.part)),
    }
}
