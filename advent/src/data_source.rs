use crate::data_file_name::DataFileNameFragment;
use clap::ValueEnum;

/// Enumerates every form of input an advent day could use.
#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum DataSource {
    /// Data source used when developing against the personalized input data.
    Input,
    /// Data source used when developing against the given sample data.
    Sample,
}

impl DataFileNameFragment for DataSource {
    fn to_data_file_name_fragment(self) -> &'static str {
        match self {
            DataSource::Input => "input",
            DataSource::Sample => "sample",
        }
    }
}
