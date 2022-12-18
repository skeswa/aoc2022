use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

/// An inclusive range of section ids that an elf is responsible for.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SectionAssignment {
    /// First section id included in this range.
    pub(crate) from: u8,
    /// Last section id included in this range.
    pub(crate) to: u8,
}

impl SectionAssignment {
    /// Returns the constructed [SectionAssignment] that best corresponds to the
    /// specified `encoded_section_assignment`.
    pub(crate) fn parse(encoded_section_assignment: &str) -> Result<SectionAssignment> {
        lazy_static! {
          /// Regular expression designed to match strings that look like
          /// " 1-2", or "34   - 7".
          static ref ENCODED_SECTION_ASSIGNMENT_PATTERN: Regex =
              Regex::new(r"^\s*(\d+)\s*-(\d+)\s*$").unwrap();
        }

        let parsed_encoded_section_assignment = ENCODED_SECTION_ASSIGNMENT_PATTERN
            .captures(encoded_section_assignment)
            .with_context(|| {
                format!(
                    "\"{}\" is not a valid encoded section assignment",
                    encoded_section_assignment
                )
            })?;

        let first_chunk = parsed_encoded_section_assignment.get(1).unwrap().as_str();
        let second_chunk = parsed_encoded_section_assignment.get(2).unwrap().as_str();

        let from = first_chunk.parse::<u8>()?;
        let to = second_chunk.parse::<u8>()?;

        Ok(SectionAssignment { from: from, to: to })
    }

    /// Returns `true` if this [SectionAssignment] fully contains the `other`
    /// one.
    pub(crate) fn fully_contains(&self, other: &Self) -> bool {
        self.from <= other.from && self.to >= other.to
    }
}
