use crate::data_source::DataSource;
use crate::part::Part;
use clap::Parser;

/// Standard configuration for an advent day program.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Specifies where the data powering this advent day should come from.
    #[arg(default_value_t = DataSource::Sample, long, short, value_enum)]
    pub(crate) data_source: DataSource,

    /// Specifies which part is active for this advent day.
    #[arg(default_value_t = Part::One, long, short, value_enum)]
    pub(crate) part: Part,
}

impl Config {
    /// Returns a [Config] derived from the command line arguments supplied to
    /// this program.
    pub(crate) fn read_from_args() -> Config {
        Config::parse()
    }
}
