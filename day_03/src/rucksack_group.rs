use std::collections::HashSet;

use crate::rucksack::Rucksack;

/// Represents a group of three rucksacks.
#[derive(Debug)]
pub(crate) struct RucksackGroup<'a> {
    /// Set of all the item types shared by all of the rucksacks.
    pub(crate) collisions: HashSet<char>,
    /// All of the rucksacks in this group.
    #[allow(dead_code)]
    pub(crate) rucksacks: Vec<&'a Rucksack>,
}

impl<'a> RucksackGroup<'a> {
    /// Assembles a new [RucksackGroup] from a specified collection of `rucksacks`.
    pub(crate) fn new(rucksacks: impl Iterator<Item = &'a Rucksack>) -> RucksackGroup<'a> {
        let cloned_rucksacks = rucksacks.collect::<Vec<&'a Rucksack>>();

        let collisions = cloned_rucksacks
            .iter()
            .fold(None, |maybe_acc: Option<HashSet<char>>, rucksack| {
                let item_types = rucksack
                    .item_types
                    .iter()
                    .map(|item_type| item_type.to_owned())
                    .collect(); // horrible

                maybe_acc
                    .map(|acc| {
                        acc.intersection(&item_types)
                            .map(|item_type| item_type.to_owned())
                            .collect()
                    })
                    .or(Some(item_types))
            })
            .unwrap_or(HashSet::new());

        RucksackGroup {
            collisions: collisions,
            rucksacks: cloned_rucksacks,
        }
    }
}
