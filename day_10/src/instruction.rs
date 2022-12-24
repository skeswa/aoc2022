use anyhow::{anyhow, Context, Result};

use lazy_static::lazy_static;
use regex::Regex;

/// Enumerates every variety of command.
///
/// Each command includes its input and output.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Instruction {
    /// Instruction that adds an integer to the current register value.
    Add(i64),
    /// Instruction that does nothing.
    NoOp,
}

impl Instruction {
    /// Attempts to interpret `encoded_instruction` as an [Instruction],
    /// returning [Err] if interpretation fails.
    pub(crate) fn parse(encoded_instruction: &str) -> Result<Instruction> {
        lazy_static! {
          /// Regular expression designed to match strings that look like
          /// " addv -34 ".
          static ref ENCODED_ADD_PATTERN: Regex =
              Regex::new(r"^\s*addx\s+(-?\d+)\s*$").unwrap();

          /// Regular expression designed to match strings that look like
          /// "noop ".
          static ref ENCODED_NOOP_PATTERN: Regex =
              Regex::new(r"^\s*noop\s*$").unwrap();
        }

        if let Some(encoded_add) = ENCODED_ADD_PATTERN.captures(&encoded_instruction) {
            let raw_integer = encoded_add.get(1).unwrap().as_str();

            let integer = raw_integer
                .parse::<i64>()
                .with_context(|| format!("\"{}\" is not a valid integer", raw_integer))?;

            Ok(Instruction::Add(integer))
        } else if ENCODED_NOOP_PATTERN.is_match(encoded_instruction) {
            Ok(Instruction::NoOp)
        } else {
            Err(anyhow!(
                "\"{}\" is not a valid instruction",
                encoded_instruction
            ))
        }
    }
}
