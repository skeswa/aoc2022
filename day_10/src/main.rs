mod computer;
mod instruction;

extern crate advent;
extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

use anyhow::{Context, Result};
use instruction::Instruction;

use computer::Computer;

#[tokio::main]
async fn main() -> Result<()> {
    let config = advent::begin();

    let encoded_instructions = advent::data(&config).await?;

    let instructions = encoded_instructions
        .lines()
        .map(|encoded_instruction| Instruction::parse(encoded_instruction))
        .collect::<Result<Vec<Instruction>>>()
        .context("Failed to interpret instructions")?;

    let mut i = 0;
    let mut total_signal_strength: i64 = 0;

    let mut computer = Computer::new(|signal_strength| {
        if i < 6 {
            total_signal_strength = total_signal_strength + signal_strength;
        }

        i = i + 1;
    });

    for instruction in instructions {
        computer.compute(instruction);
    }

    println!("Total signal strength: {}", total_signal_strength);

    Ok(())
}
