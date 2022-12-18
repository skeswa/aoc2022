use anyhow::{anyhow, Result};

use crate::section_assignment::SectionAssignment;

/// A tuple of two [SectionAssignment] instances.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SectionAssignmentPair(SectionAssignment, SectionAssignment);

impl SectionAssignmentPair {
    /// Returns the constructed [SectionAssignmentPair] that best corresponds
    /// with the specified `encoded_section_assignment_pair`.
    pub(crate) fn parse(encoded_section_assignment_pair: &str) -> Result<SectionAssignmentPair> {
        let parts = encoded_section_assignment_pair
            .split(',')
            .collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err(anyhow!(
                "\"{}\" is not a valid encoded section assignment pair: it has {} parts",
                encoded_section_assignment_pair,
                parts.len()
            ));
        }

        let first_section_assignment = SectionAssignment::parse(parts[0])?;
        let second_section_assignment = SectionAssignment::parse(parts[1])?;

        Ok(SectionAssignmentPair(
            first_section_assignment,
            second_section_assignment,
        ))
    }

    /// Returns `true` if this pair's assignments overlap at all.
    pub(crate) fn has_overlap(&self) -> bool {
        self.0.overlaps(&self.1)
    }

    /// Returns `true` if one of this pair's assignments is fully contained by
    /// the other.
    pub(crate) fn has_rendundancy(&self) -> bool {
        self.0.fully_contains(&self.1) || self.1.fully_contains(&self.0)
    }
}
