extern crate advent;
extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod priorities;
mod rucksack;

use anyhow::{Context, Result};
use priorities::priority_of;
use rucksack::Rucksack;

#[tokio::main]
async fn main() -> Result<()> {
    let config = advent::begin();

    let supplies_list = advent::data(&config).await?;

    let rucksacks = supplies_list
        .lines()
        .map(|encoded_rucksack| Rucksack::parse(encoded_rucksack))
        .collect::<Result<Vec<Rucksack>>>()
        .context("Failed to read supplies list")?;

    let priority_total = rucksacks
        .iter()
        .flat_map(|rucksack| rucksack.collisions.iter())
        .map(|collision| priority_of(collision).unwrap_or(&0).to_owned())
        .fold(0, |acc: u32, priority| acc + u32::from(priority));

    println!("Priority total: {}", priority_total);

    Ok(())
}
