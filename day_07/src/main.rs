extern crate advent;
extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod command_invocation;
mod directory_entry;
mod shell_exchange;

use anyhow::{Context, Result};
use command_invocation::CommandInvocation;
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

    println!("Hello advent! {:#?}", command_invocations);

    Ok(())
}
