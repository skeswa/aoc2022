extern crate advent;
extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod data_stream;

use anyhow::{Context, Result};
use data_stream::DataStream;

#[tokio::main]
async fn main() -> Result<()> {
    let config = advent::begin();

    let encoded_data_stream = advent::data(&config).await?;

    let start_packet_index = DataStream::parse(&encoded_data_stream)
        .start_packet_index()
        .context("Encoded data stream did not have a start packet marker")?;

    println!("Start packet index: {}", start_packet_index);

    Ok(())
}
