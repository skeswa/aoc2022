extern crate advent;
extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod command_invocation;
mod directory_entry;
mod file_system;
mod shell_exchange;

use anyhow::{Context, Result};
use command_invocation::CommandInvocation;
use file_system::FileSystem;
use lazy_static::lazy_static;
use regex::Regex;
use shell_exchange::ShellExchange;

#[tokio::main]
async fn main() -> Result<()> {
    let config = advent::begin();

    let terminal_output = advent::data(&config).await?;

    let command_invocations = terminal_output
        .split("$")
        .filter(|encoded_shell_exchange| {
            lazy_static! {
              /// Regular expression designed to match lines filled with
              /// whitespace.
              static ref WHITESPACE_LINE_PATTERN: Regex =
                  Regex::new(r"^\s*$").unwrap();
            }

            !WHITESPACE_LINE_PATTERN.is_match(encoded_shell_exchange)
        })
        .map(|encoded_shell_exchange| ShellExchange::parse(encoded_shell_exchange))
        .collect::<Result<Vec<ShellExchange>>>()
        .context("Failed to read shell exchanges")?
        .iter()
        .map(|shell_exchange| CommandInvocation::from(shell_exchange))
        .collect::<Result<Vec<CommandInvocation>>>()
        .context("Failed to read command invocations")?;

    let file_system = FileSystem::build_imperatively(command_invocations)
        .context("Failed to assemble file system from imperative commands")?;

    let mut file_system_sizes = file_system.sizes();

    match config.part {
        1 => {
            let total_size_of_slender_directories = file_system_sizes
                .iter()
                .filter(|(node, size)| node.is_directory() && *size <= 100000)
                .fold(0, |acc, (_, size)| acc + *size);

            println!(
                "Total size of all directories smaller than 100000: {}",
                total_size_of_slender_directories
            );
        }
        _ => {
            let total_size = file_system_sizes
                .iter()
                .rev()
                .filter(|(node, _)| node.name == "/")
                .map(|(_, size)| *size)
                .next()
                .context("Failed to derive \"/\" size total")?;
            println!("Total size: {}", total_size);

            let remaining_space = 70_000_000 - total_size;
            println!("Remaining space: {}", remaining_space);

            file_system_sizes.sort_by(|a, b| a.1.cmp(&b.1));

            let size_of_directory_to_delete = file_system_sizes
                .iter()
                .filter(|(node, size)| {
                    node.is_directory() && (remaining_space + *size) >= 30_000_000
                })
                .map(|(_, size)| *size)
                .next()
                .context("Could find a directory to delete")?;

            println!(
                "We can delete directory with size: {}",
                size_of_directory_to_delete
            );
        }
    }

    Ok(())
}
