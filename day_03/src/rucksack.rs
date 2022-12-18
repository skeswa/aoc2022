use std::collections::HashSet;

use anyhow::{anyhow, Result};

/// Represents one rucksack containing supplies for the jungle journey.
#[derive(Debug)]
pub(crate) struct Rucksack {
    /// Set of all the item types shared by any compartments.
    pub(crate) collisions: HashSet<char>,
    /// Set of all the item types to appear in this [Rucksack].
    pub(crate) item_types: HashSet<char>,
}

impl Rucksack {
    /// Compiles a new [Rucksack] from its encoded form.
    pub(crate) fn parse(encoded_rucksack: &str) -> Result<Rucksack> {
        let item_types = encoded_rucksack.chars().collect::<Vec<char>>();

        if item_types.len() < 2 {
            return Err(anyhow!(
                "\"{}\" is not a valid rucksack: it is not long enough",
                encoded_rucksack
            ));
        }
        if item_types.len() % 2 == 1 {
            return Err(anyhow!(
                "\"{}\" is not a valid rucksack: it is not even length string",
                encoded_rucksack
            ));
        }

        let halfway = item_types.len() / 2;

        let first_compartment = item_types[0..halfway]
            .iter()
            .map(|item_type| item_type.to_owned())
            .collect::<HashSet<char>>();
        let second_compartment = item_types[halfway..]
            .iter()
            .map(|item_type| item_type.to_owned())
            .collect::<HashSet<char>>();

        let collisions = first_compartment
            .intersection(&second_compartment)
            .map(|collision| collision.to_owned())
            .collect::<HashSet<char>>();

        Ok(Rucksack {
            collisions: collisions,
            item_types: item_types
                .iter()
                .map(|item_type| item_type.to_owned())
                .collect::<HashSet<char>>(),
        })
    }
}
