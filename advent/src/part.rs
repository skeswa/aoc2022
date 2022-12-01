use crate::data_file_name::DataFileNameFragment;
use clap::ValueEnum;

/// Enumerates each part of an advent day.
#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Part {
    /// First part of an advent day.
    One,
    /// Second part of an advent day.
    Two,
}

impl DataFileNameFragment for Part {
    fn to_data_file_name_fragment(self) -> &'static str {
        match self {
            Part::One => "1",
            Part::Two => "2",
        }
    }
}
