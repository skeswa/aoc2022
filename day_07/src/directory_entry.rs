use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

/// Enumerates everything that can be in a directory.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DirectoryEntry {
    /// Describes a file in a directory.
    File {
        /// Name of the file.
        name: String,
        /// How many bytes the file accounts for.
        size: u32,
    },
    /// Describes a directory in a directory.
    Subdirectory {
        /// Name of the directory.
        name: String,
    },
}

impl DirectoryEntry {
    /// Interprets `encoded_directory_entry` as a [DirectoryEntry], returning
    /// [Err] if that isn't possible.
    pub(crate) fn parse(encoded_directory_entry: &str) -> Result<DirectoryEntry> {
        lazy_static! {
            /// Regular expression designed to match strings that look like
            /// " dir xyz "
            static ref DIRECTORY_PATTERN: Regex =
                Regex::new(r"^\s*dir\s+([^\s]+).*$").unwrap();

            /// Regular expression designed to match strings that look like
            /// "12345678 abc.xyz"
            static ref FILE_PATTERN: Regex =
                Regex::new(r"^\s*(\d+)\s+([^\s]+).*$").unwrap();
        }

        if let Some(directory_entry) = FILE_PATTERN.captures(encoded_directory_entry) {
            let name = directory_entry.get(2).unwrap().as_str().to_owned();
            let raw_size = directory_entry.get(1).unwrap().as_str().to_owned();

            let size = raw_size
                .parse::<u32>()
                .with_context(|| format!("\"{}\" is not a valid file size", raw_size))?;

            return Ok(DirectoryEntry::File {
                name: name,
                size: size,
            });
        }

        if let Some(directory_entry) = DIRECTORY_PATTERN.captures(encoded_directory_entry) {
            let name = directory_entry.get(1).unwrap().as_str().to_owned();

            return Ok(DirectoryEntry::Subdirectory { name: name });
        }

        return Err(anyhow!(
            "\"{}\" is not a valid directory entry",
            encoded_directory_entry
        ));
    }
}
