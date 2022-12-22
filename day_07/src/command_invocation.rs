use anyhow::{anyhow, Context, Result};

use crate::{directory_entry::DirectoryEntry, shell_exchange::ShellExchange};
use lazy_static::lazy_static;
use regex::Regex;

/// Enumerates every variety of command.
///
/// Each command includes its input and output.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CommandInvocation {
    /// Command that changes the current directory path to `directory_path`.
    ChangeDirectory {
        /// Path to the directory that this command moved the shell into.
        directory_path: String,
    },
    /// Command that lists the contents of the current directory.
    ListDirectoryContents {
        /// Entries within the current directory.
        directory_entries: Vec<DirectoryEntry>,
    },
}

impl CommandInvocation {
    /// Interprets `shell_exchange` as a [CommandInvocation], returning [Err] if
    /// that is impossible.
    pub(crate) fn from(shell_exchange: &ShellExchange) -> Result<CommandInvocation> {
        lazy_static! {
          /// Regular expression designed to match strings that look like
          /// " cd  .. "
          static ref ENCODED_CD_PATTERN: Regex =
              Regex::new(r"^\s*cd\s+([^\s]+).*$").unwrap();

          /// Regular expression designed to match strings that look like
          /// " ls"
          static ref ENCODED_LS_PATTERN: Regex =
              Regex::new(r"^\s*ls\s*$").unwrap();
        }

        if let Some(cd) = ENCODED_CD_PATTERN.captures(&shell_exchange.input) {
            let directory_path = cd.get(1).unwrap().as_str().to_owned();

            return Ok(CommandInvocation::ChangeDirectory {
                directory_path: directory_path,
            });
        }

        if ENCODED_LS_PATTERN.is_match(&shell_exchange.input) {
            let directory_entries = shell_exchange
                .output
                .iter()
                .map(|output_line| DirectoryEntry::parse(output_line))
                .collect::<Result<Vec<DirectoryEntry>>>()
                .context("Failed to parse directory entries")?;

            return Ok(CommandInvocation::ListDirectoryContents {
                directory_entries: directory_entries,
            });
        }

        Err(anyhow!(
            "Unrecognized command invocation: \"{}\"",
            shell_exchange.input
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_interprets_shell_exchanges_correctly() {
        assert_eq!(
            CommandInvocation::from(&ShellExchange {
                input: "cd .. ".to_string(),
                output: vec![]
            })
            .unwrap(),
            CommandInvocation::ChangeDirectory {
                directory_path: "..".to_string()
            },
        );

        assert_eq!(
            CommandInvocation::from(&ShellExchange {
                input: " cd ./a/b/c".to_string(),
                output: vec![]
            })
            .unwrap(),
            CommandInvocation::ChangeDirectory {
                directory_path: "./a/b/c".to_string()
            },
        );

        assert_eq!(
            CommandInvocation::from(&ShellExchange {
                input: "ls".to_string(),
                output: vec![
                    "123 a.txt ".to_string(),
                    "dir o_shit_waddup".to_string(),
                    "  798123 foo_bar.png".to_string(),
                ],
            })
            .unwrap(),
            CommandInvocation::ListDirectoryContents {
                directory_entries: vec![
                    DirectoryEntry::File {
                        name: "a.txt".to_string(),
                        size: 123,
                    },
                    DirectoryEntry::Subdirectory {
                        name: "o_shit_waddup".to_string()
                    },
                    DirectoryEntry::File {
                        name: "foo_bar.png".to_string(),
                        size: 798123,
                    },
                ],
            },
        );
    }
}
