extern crate anyhow;
extern crate clap;
extern crate tokio;

mod config;
mod data_file_name;
mod data_source;
mod part;

pub use config::Config;
pub use data_file_name::*;
pub use data_source::DataSource;
pub use part::Part;

use anyhow::{Context, Ok, Result};
use std::env::current_dir;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

/// Starts this advent day, returning the specified [Config].
pub fn begin() -> Config {
    Config::read_from_args()
}

/// Reads the input data for the advent day configured by `config`.
pub async fn data(config: &Config) -> Result<String> {
    let pwd = current_dir().context("Failed to read current working directory")?;

    let data_file_name = (&config.data_source, &config.part).to_data_file_name();

    let data_file_path = pwd.join("files").join(data_file_name);

    let mut data_file = File::open(&data_file_path).await.with_context(|| {
        format!(
            "Failed to open file at path \"{}\"",
            data_file_path.display()
        )
    })?;
    let mut raw_data_file_contents = vec![];

    data_file
        .read_to_end(&mut raw_data_file_contents)
        .await
        .with_context(|| {
            format!(
                "Failed to read file at path \"{}\"",
                data_file_path.display()
            )
        })?;

    let data_file_contents = String::from_utf8_lossy(&raw_data_file_contents);

    Ok(data_file_contents.to_string())
}
