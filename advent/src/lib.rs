extern crate anyhow;
extern crate clap;
extern crate tokio;

mod config;
mod data_file_name;
mod data_source;

pub use config::Config;
pub use data_file_name::*;
pub use data_source::DataSource;

use anyhow::{anyhow, Context, Ok, Result};
use std::env::current_dir;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

/// Starts this advent day, returning the specified [Config].
pub fn begin() -> Config {
    Config::read_from_args()
}

/// Reads the input data for the advent day configured by `config`.
pub async fn data(config: &Config) -> Result<String> {
    let pwd = current_dir().context("Failed to read current working directory")?;

    let data_file_names = (&config.data_source, &config.part).to_data_file_names();

    let data_file_paths = data_file_names
        .iter()
        .map(|data_file_name| pwd.join("files").join(data_file_name))
        .collect::<Vec<PathBuf>>();

    let data_file = try_read_data_files(data_file_paths).await?;

    Ok(String::from_utf8_lossy(&data_file).to_string())
}

/// Reads each of the specified `data_file_paths` returning the first one that
/// exists.
async fn try_read_data_files(data_file_paths: Vec<PathBuf>) -> Result<Vec<u8>> {
    for i in 0..data_file_paths.len() {
        let data_file_path = &data_file_paths[i];

        let maybe_data_file = File::open(&data_file_path).await.with_context(|| {
            format!(
                "Failed to open file at path \"{}\"",
                data_file_path.display()
            )
        });

        match maybe_data_file {
            std::result::Result::Ok(mut data_file) => {
                let mut raw_data_file_contents = vec![];

                data_file.read_to_end(&mut raw_data_file_contents).await?;

                return Ok(raw_data_file_contents);
            }
            std::result::Result::Err(error) => {
                let is_last_path = i == data_file_paths.len() - 1;
                if is_last_path {
                    return Err(error);
                }
            }
        }
    }

    Err(anyhow!(
        "Failed to open any of the files at paths \"{:?}\"",
        data_file_paths
    ))
}

impl DataFileNameFragment for u8 {
    fn to_data_file_name_fragment(self) -> String {
        self.to_string()
    }
}
