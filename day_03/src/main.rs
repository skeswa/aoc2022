extern crate advent;
extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod priorities;
mod rucksack;
mod rucksack_group;

use anyhow::{Context, Result};
use priorities::priority_of;
use rucksack::Rucksack;
use rucksack_group::RucksackGroup;

#[tokio::main]
async fn main() -> Result<()> {
    let config = advent::begin();

    let supplies_list = advent::data(&config).await?;

    let rucksacks = supplies_list
        .lines()
        .map(|encoded_rucksack| Rucksack::parse(encoded_rucksack))
        .collect::<Result<Vec<Rucksack>>>()
        .context("Failed to read supplies list")?;

    let all_collisions = match config.part {
        1 => rucksacks
            .iter()
            .flat_map(|rucksack| rucksack.collisions.iter())
            .map(|collision_char| collision_char.to_owned())
            .collect::<Vec<char>>(),
        2 => {
            let rucksack_groups = rucksacks
                .chunks(3)
                .map(|rucksacks| RucksackGroup::new(rucksacks.iter()))
                .collect::<Vec<RucksackGroup>>();

            rucksack_groups
                .iter()
                .flat_map(|x| x.collisions.iter())
                .map(|collision_char| collision_char.to_owned())
                .collect::<Vec<char>>()
        }
        _ => todo!(),
    };

    let priority_total = all_collisions
        .iter()
        .map(|collision| priority_of(collision).unwrap_or(&0).to_owned())
        .fold(0, |acc: u32, priority| acc + u32::from(priority));

    println!("Priority total: {}", priority_total);

    Ok(())
}
